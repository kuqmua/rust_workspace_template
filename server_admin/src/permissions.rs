use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminPermissionsPageError)]
pub(crate) async fn permissions(auth: crate::AdminAuthReq) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(server_admin_contract::domain_types::AdminDataTable::Permissions),
    )
    .await
}
