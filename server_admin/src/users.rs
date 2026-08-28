use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminUsersPageError)]
pub(crate) async fn users(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Users),
    )
    .await
}
