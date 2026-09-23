#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_permission_actions_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let query = crate::admin_table_query::admin_table_query(admin_csr_query)?;
    let request = server_admin_contract::admin_permission_actions_read_request::AdminPermissionActionsReadRequest::try_from(&query)?;
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::PermissionActions,
    )
    .await
}
