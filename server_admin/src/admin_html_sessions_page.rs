use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminSessionsPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn admin_html_sessions_page(
    auth: crate::AdminAuthReq,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Sessions,
        None,
    )
    .await
}
