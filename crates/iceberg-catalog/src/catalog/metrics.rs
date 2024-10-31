use crate::request_metadata::RequestMetadata;
use crate::rest::iceberg::v1::{ApiContext, Result, TableParameters};

use crate::modules::{authz::Authorizer, secrets::SecretStore, Catalog, State};

use super::CatalogServer;

#[async_trait::async_trait]
impl<C: Catalog, A: Authorizer + Clone, S: SecretStore>
    crate::rest::iceberg::v1::metrics::Service<State<A, C, S>> for CatalogServer<C, A, S>
{
    async fn report_metrics(
        _: TableParameters,
        _: serde_json::Value,
        _: ApiContext<State<A, C, S>>,
        _: RequestMetadata,
    ) -> Result<()> {
        Ok(())
    }
}
