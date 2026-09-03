#[proc_macro_frontend_contract::route_error(AdminPermissionsPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "permissions remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn permissions(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        admin_auth_request,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(server_admin_contract::admin_data_table::AdminDataTable::Permissions),
    )
    .await
}
