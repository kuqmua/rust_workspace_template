#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_user_roles_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let request = server_admin_contract::admin_data_table_query::AdminDataTableQuery::new(
        crate::admin_identifier_filter_query::admin_identifier_filter_query(admin_csr_query)?,
        crate::admin_table_query::admin_table_query(admin_csr_query)?,
    );
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::UserRolesTable,
    )
    .await
}
