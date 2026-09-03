#[proc_macro_frontend_contract::route_error(AdminRolesPageError)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn roles(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        admin_auth_request,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(server_admin_contract::admin_data_table::AdminDataTable::Roles),
    )
    .await
}
