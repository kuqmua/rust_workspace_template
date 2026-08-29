#[frontend_contract_macros::route_error(AdminSessionsPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn admin_html_sessions_page(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        auth,
        server_admin_contract::admin_page::AdminPage::Sessions,
        None,
    )
    .await
}
