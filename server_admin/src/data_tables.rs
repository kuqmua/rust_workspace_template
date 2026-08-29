#[frontend_contract_macros::route_error(AdminDataTablesPageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn data_tables(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_path::AxumAdminPath(table): crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_data_table::AdminDataTable,
    >,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        auth,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(table),
    )
    .await
}
