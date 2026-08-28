use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminSettingsPageError)]
pub(crate) async fn settings(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Settings,
        None,
    )
    .await
}
