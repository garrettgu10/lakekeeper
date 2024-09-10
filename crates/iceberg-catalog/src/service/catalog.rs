use super::{
    storage::StorageProfile, NamespaceIdentUuid, ProjectIdent, TableIdentUuid, WarehouseIdent,
    WarehouseStatus,
};
pub use crate::api::iceberg::v1::{
    CreateNamespaceRequest, CreateNamespaceResponse, ListNamespacesQuery, ListNamespacesResponse,
    NamespaceIdent, Result, TableIdent, UpdateNamespacePropertiesRequest,
    UpdateNamespacePropertiesResponse,
};
use crate::api::iceberg::v1::{PaginatedTabulars, PaginationQuery};
use crate::service::health::HealthExt;
use crate::SecretIdent;

use crate::api::management::v1::warehouse::TabularDeleteProfile;
use crate::service::tabular_idents::{TabularIdentOwned, TabularIdentUuid};
use iceberg::spec::{TableMetadata, ViewMetadata};
use iceberg_ext::catalog::rest::ErrorModel;
pub use iceberg_ext::catalog::rest::{CommitTableResponse, CreateTableRequest};
use iceberg_ext::configs::Location;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Transaction<D>
where
    Self: Sized + Send + Sync,
{
    type Transaction<'a>
    where
        Self: 'a;

    async fn begin_write(db_state: D) -> Result<Self>;

    async fn begin_read(db_state: D) -> Result<Self>;

    async fn commit(self) -> Result<()>;

    async fn rollback(self) -> Result<()>;

    fn transaction(&mut self) -> Self::Transaction<'_>;
}

#[derive(Debug)]
pub struct GetNamespaceResponse {
    /// Reference to one or more levels of a namespace
    pub namespace: NamespaceIdent,
    pub namespace_id: NamespaceIdentUuid,
    pub warehouse_id: WarehouseIdent,
    pub properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug)]
pub struct CreateTableResponse {
    pub table_metadata: TableMetadata,
}

#[derive(Debug)]
pub struct LoadTableResponse {
    pub table_id: TableIdentUuid,
    pub namespace_id: NamespaceIdentUuid,
    pub table_metadata: TableMetadata,
    pub metadata_location: Option<String>,
    pub storage_secret_ident: Option<SecretIdent>,
    pub storage_profile: StorageProfile,
}

#[derive(Debug)]
pub struct GetTableMetadataResponse {
    pub table: TableIdent,
    pub table_id: TableIdentUuid,
    pub warehouse_id: WarehouseIdent,
    pub location: String,
    pub metadata_location: Option<String>,
    pub storage_secret_ident: Option<SecretIdent>,
    pub storage_profile: StorageProfile,
}

#[derive(Debug)]
pub struct GetStorageConfigResponse {
    pub storage_profile: StorageProfile,
    pub storage_secret_ident: Option<SecretIdent>,
}

#[derive(Debug, Clone)]
pub struct Warehouse {
    /// ID of the warehouse.
    pub id: WarehouseIdent,
    /// Name of the warehouse.
    pub name: String,
    /// Project ID in which the warehouse is created.
    pub project_id: ProjectIdent,
    /// Storage profile used for the warehouse.
    pub storage_profile: StorageProfile,
    /// Storage secret ID used for the warehouse.
    pub storage_secret_id: Option<SecretIdent>,
    /// Whether the warehouse is active.
    pub status: WarehouseStatus,
    /// Tabular delete profile used for the warehouse.
    pub tabular_delete_profile: TabularDeleteProfile,
}

impl Warehouse {
    pub(crate) fn base_location(&self) -> Result<Location> {
        let mut location = self.storage_profile.base_location()?;
        location.without_trailing_slash().push(&self.id.to_string());
        Ok(location)
    }

    #[must_use]
    pub fn is_allowed_location(&self, other: &Location) -> bool {
        let base_location = self.base_location().ok();

        if let Some(mut base_location) = base_location {
            base_location.with_trailing_slash();
            if other == &base_location {
                return false;
            }

            other.is_sublocation_of(&base_location)
        } else {
            false
        }
    }

    // allowed table location is: '{warehouse_location}/{namespace_id}/{table_id}'
    pub(crate) fn check_allowed_table_location(
        &self,
        namespace: &GetNamespaceResponse,
        other: &Location,
    ) -> Result<Uuid> {
        let mut base_location = self.base_location()?;
        base_location.with_trailing_slash();

        if !other.is_sublocation_of(&base_location) || other == &base_location {
            return Err(ErrorModel::bad_request("Table location is not allowed. Table location cannot be the same as the warehouse location.", "InvalidTableLocation", None ).into());
        }
        let other = other.lstrip(base_location.as_str());
        let without_base = other.trim_start_matches('/');
        let parts = without_base.split('/').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(ErrorModel::bad_request("Table location is not allowed. Table location has to be of format '{warehouse_location}/{namespace_id}/{table_id}'.", "InvalidTableLocation", None ).into());
        }

        let Ok(namespace_location) = parts[0].parse::<Uuid>() else {
            return Err(ErrorModel::bad_request("Table location is not allowed. Table location has to be of format '{warehouse_location}/{namespace_id}/{table_id}'.", "InvalidTableLocation", None ).into());
        };

        if *namespace.namespace_id != namespace_location {
            return Err(ErrorModel::bad_request("Table location is not allowed. Table location has to be of format '{warehouse_location}/{namespace_id}/{table_id}'.", "InvalidTableLocation", None ).into());
        }

        let Ok(table_id) = parts[1].parse::<Uuid>() else {
            return Err(ErrorModel::bad_request("Table location is not allowed. Table location has to be of format '{warehouse_location}/{namespace_id}/{table_id}'.", "InvalidTableLocation", None ).into());
        };

        Ok(table_id)
    }
}

#[derive(Debug, Clone)]
pub struct TableCommit {
    pub new_metadata: TableMetadata,
    pub new_metadata_location: Location,
}

#[async_trait::async_trait]
pub trait Catalog
where
    Self: Clone + Send + Sync + 'static,
{
    type Transaction: Transaction<Self::State>;
    type State: Clone + Send + Sync + 'static + HealthExt;

    // Should only return namespaces if the warehouse is active.
    async fn list_namespaces(
        warehouse_id: WarehouseIdent,
        query: &ListNamespacesQuery,
        catalog_state: Self::State,
    ) -> Result<ListNamespacesResponse>;

    async fn create_namespace<'a>(
        warehouse_id: WarehouseIdent,
        namespace_id: NamespaceIdentUuid,
        request: CreateNamespaceRequest,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<CreateNamespaceResponse>;

    // Should only return a namespace if the warehouse is active.
    async fn get_namespace<'a>(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<GetNamespaceResponse>;

    /// Return Err only on unexpected errors, not if the namespace does not exist.
    /// If the namespace does not exist, return Ok(false).
    ///
    /// We use this function also to handle the `namespace_exists` endpoint.
    /// Also return Ok(false) if the warehouse is not active.
    async fn namespace_ident_to_id(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        catalog_state: Self::State,
    ) -> Result<Option<NamespaceIdentUuid>>;

    async fn drop_namespace<'a>(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Update the properties of a namespace.
    ///
    /// The properties are the final key-value properties that should
    /// be persisted as-is in the catalog.
    async fn update_namespace_properties<'a>(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        properties: HashMap<String, String>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    async fn create_table<'a>(
        namespace_id: NamespaceIdentUuid,
        table: &TableIdent,
        table_id: TableIdentUuid,
        request: CreateTableRequest,
        // Metadata location may be none if stage-create is true
        metadata_location: Option<&str>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<CreateTableResponse>;

    async fn list_tables(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        list_flags: ListFlags,
        catalog_state: Self::State,
        pagination_query: PaginationQuery,
    ) -> Result<PaginatedTabulars<TableIdentUuid, TableIdent>>;

    /// Return Err only on unexpected errors, not if the table does not exist.
    /// If include_staged is true, also return staged tables.
    /// If the table does not exist, return Ok(None).
    ///
    /// We use this function also to handle the `table_exists` endpoint.
    /// Also return Ok(None) if the warehouse is not active.
    async fn table_ident_to_id(
        warehouse_id: WarehouseIdent,
        table: &TableIdent,
        list_flags: ListFlags,
        catalog_state: Self::State,
    ) -> Result<Option<TableIdentUuid>>;

    /// Same as `table_ident_to_id`, but for multiple tables.
    async fn table_idents_to_ids(
        warehouse_id: WarehouseIdent,
        tables: HashSet<&TableIdent>,
        list_flags: ListFlags,
        catalog_state: Self::State,
    ) -> Result<HashMap<TableIdent, Option<TableIdentUuid>>>;

    /// Load tables by table id.
    /// Does not return staged tables.
    /// If a table does not exist, do not include it in the response.
    async fn load_tables<'a>(
        warehouse_id: WarehouseIdent,
        tables: impl IntoIterator<Item = TableIdentUuid> + Send,
        include_deleted: bool,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<HashMap<TableIdentUuid, LoadTableResponse>>;

    /// Get table metadata by table id.
    /// If include_staged is true, also return staged tables,
    /// i.e. tables with no metadata file yet.
    async fn get_table_metadata_by_id(
        warehouse_id: WarehouseIdent,
        table: TableIdentUuid,
        list_flags: ListFlags,
        catalog_state: Self::State,
    ) -> Result<GetTableMetadataResponse>;

    /// Get table metadata by location.
    async fn get_table_metadata_by_s3_location(
        warehouse_id: WarehouseIdent,
        location: &str,
        list_flags: ListFlags,
        catalog_state: Self::State,
    ) -> Result<GetTableMetadataResponse>;

    /// Rename a table. Tables may be moved across namespaces.
    async fn rename_table<'a>(
        warehouse_id: WarehouseIdent,
        source_id: TableIdentUuid,
        source: &TableIdent,
        destination: &TableIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Drop a table.
    /// Should drop staged and non-staged tables.
    ///
    /// Consider in your implementation to implement an UNDROP feature.
    ///
    /// Returns the table location
    async fn drop_table<'a>(
        table_id: TableIdentUuid,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<String>;

    async fn mark_tabular_as_deleted(
        table_id: TabularIdentUuid,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'_>,
    ) -> Result<()>;

    /// Commit changes to a table.
    /// The table might be staged or not.
    async fn commit_table_transaction<'a>(
        warehouse_id: WarehouseIdent,
        commits: impl IntoIterator<Item = TableCommit> + Send,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    // ---------------- Warehouse Management API ----------------

    /// Create a warehouse.
    async fn create_warehouse<'a>(
        warehouse_name: String,
        project_id: ProjectIdent,
        storage_profile: StorageProfile,
        tabular_delete_profile: TabularDeleteProfile,
        storage_secret_id: Option<SecretIdent>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<WarehouseIdent>;

    /// Return a list of all project ids in the catalog
    async fn list_projects(catalog_state: Self::State) -> Result<HashSet<ProjectIdent>>;

    /// Return a list of all warehouse in a project
    async fn list_warehouses(
        project_id: ProjectIdent,
        // If None, return only active warehouses
        // If Some, return only warehouses with any of the statuses in the set
        include_inactive: Option<Vec<WarehouseStatus>>,
        // If None, return all warehouses in the project
        // If Some, return only the warehouses in the set
        warehouse_id_filter: Option<&HashSet<WarehouseIdent>>,
        catalog_state: Self::State,
    ) -> Result<Vec<Warehouse>>;

    /// Get the warehouse metadata - should only return active warehouses.
    async fn get_warehouse<'a>(
        warehouse_id: WarehouseIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<Warehouse>;

    /// Delete a warehouse.
    async fn delete_warehouse<'a>(
        warehouse_id: WarehouseIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Rename a warehouse.
    async fn rename_warehouse<'a>(
        warehouse_id: WarehouseIdent,
        new_name: &str,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Set the status of a warehouse.
    async fn set_warehouse_status<'a>(
        warehouse_id: WarehouseIdent,
        status: WarehouseStatus,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    async fn update_storage_profile<'a>(
        warehouse_id: WarehouseIdent,
        storage_profile: StorageProfile,
        storage_secret_id: Option<SecretIdent>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Return Err only on unexpected errors, not if the table does not exist.
    /// If include_staged is true, also return staged tables.
    /// If the table does not exist, return Ok(None).
    ///
    /// We use this function also to handle the `view_exists` endpoint.
    /// Also return Ok(None) if the warehouse is not active.
    async fn view_ident_to_id(
        warehouse_id: WarehouseIdent,
        view: &TableIdent,
        catalog_state: Self::State,
    ) -> Result<Option<TableIdentUuid>>;

    async fn create_view<'a>(
        namespace_id: NamespaceIdentUuid,
        view: &TableIdent,
        request: ViewMetadata,
        metadata_location: &str,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    async fn load_view<'a>(
        view_id: TableIdentUuid,
        include_deleted: bool,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<ViewMetadataWithLocation>;

    async fn list_views(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        include_deleted: bool,
        catalog_state: Self::State,
        pagination_query: PaginationQuery,
    ) -> Result<PaginatedTabulars<TableIdentUuid, TableIdent>>;

    async fn update_view_metadata(
        namespace_id: NamespaceIdentUuid,
        view_id: TableIdentUuid,
        view: &TableIdent,
        metadata_location: &str,
        metadata: ViewMetadata,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'_>,
    ) -> Result<()>;

    async fn drop_view<'a>(
        view_id: TableIdentUuid,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<String>;

    async fn rename_view(
        warehouse_id: WarehouseIdent,
        source_id: TableIdentUuid,
        source: &TableIdent,
        destination: &TableIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'_>,
    ) -> Result<()>;

    async fn list_tabulars(
        warehouse_id: WarehouseIdent,
        list_flags: ListFlags,
        catalog_state: Self::State,
        pagination_query: PaginationQuery,
    ) -> Result<PaginatedTabulars<TabularIdentUuid, (TabularIdentOwned, Option<DeletionDetails>)>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ListFlags {
    pub include_active: bool,
    pub include_staged: bool,
    pub include_deleted: bool,
}

impl Default for ListFlags {
    fn default() -> Self {
        Self {
            include_active: true,
            include_staged: false,
            include_deleted: false,
        }
    }
}

impl ListFlags {
    #[must_use]
    pub fn all() -> Self {
        Self {
            include_staged: true,
            include_deleted: true,
            include_active: true,
        }
    }

    #[must_use]
    pub fn only_deleted() -> Self {
        Self {
            include_staged: false,
            include_deleted: true,
            include_active: false,
        }
    }
}

#[derive(Clone, Default, Debug, Copy, PartialEq, Eq)]
pub struct DropFlags {
    pub hard_delete: bool,
    pub purge: bool,
}

impl DropFlags {
    #[must_use]
    pub fn purge(mut self) -> Self {
        self.purge = true;
        self
    }

    #[must_use]
    pub fn hard_delete(mut self) -> Self {
        self.hard_delete = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct ViewMetadataWithLocation {
    pub metadata_location: String,
    pub metadata: ViewMetadata,
}

#[derive(Debug, Clone, Copy)]
pub struct DeletionDetails {
    pub expiration_task_id: uuid::Uuid,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    pub deleted_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod test {
    use crate::api::management::v1::warehouse::TabularDeleteProfile;
    use crate::service::storage::{S3Flavor, S3Profile, StorageProfile};
    use crate::service::{Warehouse, WarehouseStatus};
    use iceberg_ext::configs::Location;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn test_is_allowed_location() {
        let profile = StorageProfile::S3(S3Profile {
            bucket: "my.bucket".to_string(),
            endpoint: Some("http://localhost:9000".parse().unwrap()),
            region: "us-east-1".to_string(),
            assume_role_arn: None,
            path_style_access: None,
            key_prefix: Some("my/subpath".to_string()),
            sts_role_arn: None,
            sts_enabled: false,
            flavor: S3Flavor::Aws,
        });
        let warehouse = Warehouse {
            id: Uuid::now_v7().into(),
            name: "my-warehouse".to_string(),
            project_id: Uuid::now_v7().into(),
            storage_profile: profile,
            storage_secret_id: None,
            status: WarehouseStatus::Active,
            tabular_delete_profile: TabularDeleteProfile::Hard {},
        };

        let cases = vec![
            (
                format!("s3://my.bucket/my/subpath/{}/ns-id", warehouse.id),
                true,
            ),
            (
                format!("s3://my.bucket/my/subpath/{}/ns-id/", warehouse.id),
                true,
            ),
            (
                format!("s3://my.bucket/my/subpath/{}/ns-id/tbl-id", warehouse.id),
                true,
            ),
            (
                format!("s3://my.bucket/my/subpath/{}/ns-id/tbl-id/", warehouse.id),
                true,
            ),
            (
                format!(
                    "s3://other.bucket/my/subpath/{}/ns-id/tbl-id/",
                    warehouse.id
                ),
                false,
            ),
            (
                format!(
                    "s3://my.bucket/other/subpath/{}/ns-id/tbl-id/",
                    warehouse.id
                ),
                false,
            ),
            // Exact path should not be accepted
            (format!("s3://my.bucket/my/subpath/{}", warehouse.id), false),
            (
                format!("s3://my.bucket/my/subpath/{}/", warehouse.id),
                false,
            ),
        ];

        for (sublocation, expected_result) in cases {
            let sublocation = Location::from_str(&sublocation).unwrap();
            assert_eq!(
                warehouse.is_allowed_location(&sublocation),
                expected_result,
                "Base Location: {}, Maybe sublocation: {sublocation}",
                warehouse.base_location().unwrap(),
            );
        }
    }
}
