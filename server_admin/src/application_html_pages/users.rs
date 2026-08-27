use super::csr_page;

#[frontend_contract::domain_types::route_error(AdminUsersPageError)]
pub(in crate::domain_types::auth::html) async fn users(
    auth: super::super::super::AdminAuthReq,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Users),
    )
    .await
}
