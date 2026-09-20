pub(crate) async fn csr_table_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
) -> axum::response::Response {
    crate::csr_page::csr_page(
        admin_auth_request,
        server_admin_contract::admin_page::AdminPage::Tables,
        Some(admin_data_table),
    )
    .await
}
