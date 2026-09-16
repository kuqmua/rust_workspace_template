pub(crate) fn admin_table_query(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_table_query::AdminTableQuery,
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
    Ok(
        server_admin_contract::admin_table_query::AdminTableQuery::new(
            admin_csr_query.search().clone(),
            admin_csr_query.sort().clone(),
            admin_csr_query.offset(),
            admin_csr_query.limit(),
            direction,
        ),
    )
}
