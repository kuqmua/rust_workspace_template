// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::data_tables_list::data_tables_list, tag = "admin_tables")]
pub(super) async fn api_data_tables(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminDataTablesError> {
}
