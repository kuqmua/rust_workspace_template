use super::csr_page;

#[frontend_contract::domain_types::route_error(AdminProfilePageError)]
pub(in crate::domain_types::auth::html) async fn profile(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Profile,
        None,
    )
    .await
}
