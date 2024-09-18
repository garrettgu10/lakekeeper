use crate::api::iceberg::types::Prefix;
use crate::api::iceberg::v1::{DataAccess, NamespaceParameters};
use crate::api::ApiContext;
use crate::catalog::compression_codec::CompressionCodec;
use crate::catalog::io::write_metadata_file;
use crate::catalog::tables::{
    determine_tabular_location, maybe_body_to_json, require_active_warehouse,
    validate_table_or_view_ident,
};
use crate::catalog::views::validate_view_properties;
use crate::catalog::{maybe_get_secret, require_warehouse_id};
use crate::request_metadata::RequestMetadata;
use crate::service_modules::auth::AuthZHandler;
use crate::service_modules::event_publisher::EventMetadata;
use crate::service_modules::object_stores::{StorageLocations as _, StoragePermissions};
use crate::service_modules::tabular_idents::TabularIdentUuid;
use crate::service_modules::Result;
use crate::service_modules::{CatalogBackend, SecretStore, State, Transaction};
use http::StatusCode;
use iceberg::spec::ViewMetadataBuilder;
use iceberg::{TableIdent, ViewCreation};
use iceberg_ext::catalog::rest::{CreateViewRequest, ErrorModel, LoadViewResult};
use uuid::Uuid;

// TODO: split up into smaller functions
#[allow(clippy::too_many_lines)]
/// Create a view in the given namespace
pub(crate) async fn create_view<C: CatalogBackend, A: AuthZHandler, S: SecretStore>(
    parameters: NamespaceParameters,
    request: CreateViewRequest,
    state: ApiContext<State<A, C, S>>,
    data_access: DataAccess,
    request_metadata: RequestMetadata,
) -> Result<LoadViewResult> {
    // ------------------- VALIDATIONS -------------------
    let NamespaceParameters { namespace, prefix } = parameters;
    let warehouse_id = require_warehouse_id(prefix.clone())?;
    let view = TableIdent::new(namespace.clone(), request.name.clone());

    validate_table_or_view_ident(&view)?;
    validate_view_properties(request.properties.keys())?;

    if request.view_version.representations().is_empty() {
        return Err(ErrorModel::builder()
            .code(StatusCode::BAD_REQUEST.into())
            .message("View must have at least one query.".to_string())
            .r#type("EmptyView".to_string())
            .build()
            .into());
    }

    // ------------------- AUTHZ -------------------
    A::check_create_view(
        &request_metadata,
        warehouse_id,
        &namespace,
        state.v1_state.auth.clone(),
    )
    .await?;

    // ------------------- BUSINESS LOGIC -------------------
    let namespace_id =
        C::namespace_ident_to_id(warehouse_id, &namespace, state.v1_state.catalog.clone())
            .await?
            .ok_or(
                ErrorModel::builder()
                    .code(StatusCode::NOT_FOUND.into())
                    .message("Namespace does not exist".to_string())
                    .r#type("NamespaceNotFound".to_string())
                    .build(),
            )?;

    let mut t = C::Transaction::begin_write(state.v1_state.catalog.clone()).await?;
    let namespace = C::get_namespace(warehouse_id, &namespace, t.transaction()).await?;
    let warehouse = C::get_warehouse(warehouse_id, t.transaction()).await?;
    let storage_profile = warehouse.storage_profile;
    require_active_warehouse(warehouse.status)?;

    let view_id: TabularIdentUuid = TabularIdentUuid::View(uuid::Uuid::now_v7());

    let view_location = determine_tabular_location(
        &namespace,
        request.location.clone(),
        view_id,
        &storage_profile,
    )?;

    // Update the request for event
    let mut request = request;
    request.location = Some(view_location.to_string());
    let request = request; // make it immutable

    let metadata_location = storage_profile.default_metadata_location(
        &view_location,
        &CompressionCodec::try_from_properties(&request.properties)?,
        *view_id,
    );

    // serialize body before moving it
    let body = maybe_body_to_json(&request);
    let view_creation = ViewMetadataBuilder::from_view_creation(ViewCreation {
        name: view.name.clone(),
        location: view_location.to_string(),
        representations: request.view_version.representations().clone(),
        schema: request.schema,
        properties: request.properties.clone(),
        default_namespace: request.view_version.default_namespace().clone(),
        default_catalog: request.view_version.default_catalog().cloned(),
        summary: request.view_version.summary().clone(),
    })
    .unwrap()
    .assign_uuid(*view_id.as_ref());

    let metadata = view_creation.build().map_err(|e| {
        ErrorModel::bad_request(
            format!("Failed to create view metadata: {e}"),
            "ViewMetadataCreationFailed",
            Some(Box::new(e)),
        )
    })?;

    C::create_view(
        namespace_id,
        &view,
        metadata.clone(),
        &metadata_location,
        &view_location,
        t.transaction(),
    )
    .await?;

    // We don't commit the transaction yet, first we need to write the metadata file.
    let storage_secret =
        maybe_get_secret(warehouse.storage_secret_id, &state.v1_state.secrets).await?;

    let file_io = storage_profile.file_io(storage_secret.as_ref())?;
    let compression_codec = CompressionCodec::try_from_metadata(&metadata)?;
    write_metadata_file(&metadata_location, &metadata, compression_codec, &file_io).await?;
    tracing::debug!("Wrote new metadata file to: '{}'", metadata_location);

    // Generate the storage profile. This requires the storage secret
    // because the table config might contain vended-credentials based
    // on the `data_access` parameter.
    // ToDo: There is a small inefficiency here: If storage credentials
    // are not required because of i.e. remote-signing and if this
    // is a stage-create, we still fetch the secret.
    let config = storage_profile
        .generate_table_config(
            &data_access,
            storage_secret.as_ref(),
            &view_location,
            // TODO: This should be a permission based on authz
            StoragePermissions::ReadWriteDelete,
        )
        .await?;

    t.commit().await?;

    let _ = state
        .v1_state
        .publisher
        .publish(
            Uuid::now_v7(),
            "createView",
            body,
            EventMetadata {
                tabular_id: TabularIdentUuid::View(*view_id),
                warehouse_id: *warehouse_id.as_uuid(),
                name: view.name,
                namespace: view.namespace.to_url_string(),
                prefix: prefix.map(Prefix::into_string).unwrap_or_default(),
                num_events: 1,
                sequence_number: 0,
                trace_id: request_metadata.request_id,
            },
        )
        .await;

    let load_view_result = LoadViewResult {
        metadata_location: metadata_location.to_string(),
        metadata,
        config: Some(config.into()),
    };

    Ok(load_view_result)
}

#[cfg(test)]
pub(crate) mod test {
    use crate::service_modules::catalog_backends::implementations::postgres::namespace::tests::initialize_namespace;

    use crate::catalog::test::{create_view, setup};
    use iceberg::NamespaceIdent;
    use iceberg_ext::catalog::rest::CreateViewRequest;
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn test_create_view(pool: PgPool) {
        let (api_context, namespace, whi) = setup(pool, None, None, None).await;
        let whi = whi.warehouse_id;

        let mut rq = create_view_request(None, None);

        let _view = create_view(
            api_context.clone(),
            namespace.namespace.clone(),
            rq.clone(),
            Some(whi.to_string()),
        )
        .await
        .unwrap();
        let view = create_view(
            api_context.clone(),
            namespace.namespace.clone(),
            rq.clone(),
            Some(whi.to_string()),
        )
        .await
        .expect_err("Recreate with same ident should fail.");
        assert_eq!(view.error.code, 409);
        let old_name = rq.name.clone();
        rq.name = "some-other-name".to_string();

        let _view = create_view(
            api_context.clone(),
            namespace.namespace,
            rq.clone(),
            Some(whi.to_string()),
        )
        .await
        .expect("Recreate with with another name it should work");

        rq.name = old_name;
        let namespace = NamespaceIdent::from_vec(vec![Uuid::now_v7().to_string()]).unwrap();
        let new_ns = initialize_namespace(
            api_context.v1_state.catalog.clone(),
            whi.into(),
            &namespace,
            None,
        )
        .await
        .namespace;

        let _view = create_view(api_context, new_ns, rq, Some(whi.to_string()))
            .await
            .expect("Recreate with same name but different ns should work.");
    }

    pub(crate) fn create_view_request(
        name: Option<&str>,
        location: Option<&str>,
    ) -> CreateViewRequest {
        serde_json::from_value(json!({
                                  "name": name.unwrap_or("myview"),
                                  "location": location,
                                  "schema": {
                                    "schema-id": 0,
                                    "type": "struct",
                                    "fields": [
                                      {
                                        "id": 0,
                                        "name": "id",
                                        "required": false,
                                        "type": "long"
                                      }
                                    ]
                                  },
                                  "view-version": {
                                    "version-id": 1,
                                    "schema-id": 0,
                                    "timestamp-ms": 1_719_395_654_343_i64,
                                    "summary": {
                                      "engine-version": "3.5.1",
                                      "iceberg-version": "Apache Iceberg 1.5.2 (commit cbb853073e681b4075d7c8707610dceecbee3a82)",
                                      "engine-name": "spark",
                                      "app-id": "local-1719395622847"
                                    },
                                    "representations": [
                                      {
                                        "type": "sql",
                                        "sql": "select id, xyz from spark_demo.my_table",
                                        "dialect": "spark"
                                      }
                                    ],
                                    "default-namespace": []
                                  },
                                  "properties": {
                                    "create_engine_version": "Spark 3.5.1",
                                    "engine_version": "Spark 3.5.1",
                                    "spark.query-column-names": "id"
                                  }})).unwrap()
    }
}
