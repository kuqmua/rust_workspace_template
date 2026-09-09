#[proc_macro_frontend_contract_route_error::route_error(AdminHealthPageError)]
#[allow(
    clippy::single_call_fn,
    reason = "admin_health_page remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn admin_health_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        admin_auth_request,
        server_admin_contract::admin_page::AdminPage::Health,
        None,
    )
    .await
}
