use crate::csr_page;

#[frontend_contract::route_error(AdminSettingsPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn settings(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Settings,
        None,
    )
    .await
}
