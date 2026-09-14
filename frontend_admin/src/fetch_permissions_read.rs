#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_permissions_read(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_roles_page::AdminRolesPage,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let direction = match admin_csr_query.direction() {
        None => server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending,
        Some(direction)
            if direction.as_ref().as_str()
                == server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending
                    .as_ref() =>
        {
            server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending
        }
        Some(direction)
            if direction.as_ref().as_str()
                == server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
                    .as_ref() =>
        {
            server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
        }
        Some(_) => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
    };
    let admin_table_query = server_admin_contract::admin_table_query::AdminTableQuery::new(
        admin_csr_query.search().clone(),
        admin_csr_query.sort().clone(),
        admin_csr_query.offset(),
        admin_csr_query.limit(),
        direction,
    );
    let request = server_admin_contract::admin_roles_read_request::AdminRolesReadRequest::with_permissions_query(
        &admin_table_query,
    )?;
    crate::fetch_account_read_request::fetch_account_read_request(
        &request,
        server_admin_contract::admin_route::AdminRoute::Roles,
    )
    .await
}
