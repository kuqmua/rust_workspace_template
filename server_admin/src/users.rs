#[proc_macro_frontend_contract_route_error::route_error(AdminUsersPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "users remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn users(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        admin_auth_request,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(server_admin_contract::admin_data_table::AdminDataTable::Users),
    )
    .await
}
