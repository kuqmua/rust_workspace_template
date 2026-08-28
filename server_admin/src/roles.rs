use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminRolesPageError)]
pub(crate) async fn roles(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Roles),
    )
    .await
}
