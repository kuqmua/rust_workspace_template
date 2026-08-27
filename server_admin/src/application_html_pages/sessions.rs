use super::csr_page;

#[frontend_contract::domain_types::route_error(AdminSessionsPageError)]
pub(in crate::domain_types::auth::html) async fn sessions(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Sessions,
        None,
    )
    .await
}
