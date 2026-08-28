// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(
    delegate = crate::data_tables_get::data_tables_get,
    params(server_admin_contract::domain_types::AdminDataTableQuery),
    tag = "admin_tables"
)]
pub(crate) async fn api_data_table(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<server_admin_contract::domain_types::AdminDataTable>,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminDataTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminDataTableError> {
}
