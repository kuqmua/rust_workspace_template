// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(delegate = crate::data_tables_list::data_tables_list, tag = "admin_tables")]
pub(crate) async fn api_data_tables(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminDataTablesError> {
}
