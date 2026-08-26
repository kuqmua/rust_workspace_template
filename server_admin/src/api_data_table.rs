#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::data_tables_get::data_tables_get,
    params(server_admin_contract::domain_types::AdminDataTableQuery),
    tag = "admin_tables"
)]
pub(super) async fn api_data_table(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<server_admin_contract::domain_types::AdminDataTable>,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminDataTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminDataTableError> {
}
