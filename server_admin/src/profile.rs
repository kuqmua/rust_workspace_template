use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminProfilePageError)]
pub(crate) async fn profile(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Profile,
        None,
    )
    .await
}
