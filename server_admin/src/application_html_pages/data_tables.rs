use super::csr_page;

#[frontend_contract::domain_types::route_error(AdminDataTablesPageError)]
pub(in crate::domain_types::auth::html) async fn data_tables(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminPath(table): super::super::super::AxumAdminPath<
        server_admin_contract::domain_types::AdminDataTable,
    >,
) -> axum::response::Response {
    csr_page(
        auth,
        server_admin_contract::domain_types::AdminPage::Tables,
        Some(table),
    )
    .await
}
