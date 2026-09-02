// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::data_tables_list::data_tables_list, tag = "admin_tables")]
pub(crate) async fn api_data_tables(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminDataTablesError,
> {
}
