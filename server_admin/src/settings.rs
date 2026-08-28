use super::csr_page;

#[frontend_contract::domain_types::route_error(AdminSettingsPageError)]
pub(in crate::domain_types::auth::html) async fn settings(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Settings,
        None,
    )
    .await
}
