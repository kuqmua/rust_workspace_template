use crate::csr_page;

#[frontend_contract::domain_types::route_error(AdminDataTablesPageError)]
pub(crate) async fn data_tables(
    auth: crate::AdminAuthReq,
    crate::AxumAdminPath(table): crate::AxumAdminPath<
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
