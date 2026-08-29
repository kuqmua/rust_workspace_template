use super::csr_page;

#[frontend_contract::route_error(AdminUsersPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn users(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Users),
    )
    .await
}
