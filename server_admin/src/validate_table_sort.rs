pub(crate) fn validate_table_sort(
    query: &server_admin_contract::admin_table_query::AdminTableQuery,
    options: &[server_admin_contract::admin_table_sort_field::AdminTableSortField],
) -> Result<(), crate::admin_error::AdminError> {
    if query.sort().as_ref().is_empty() {
        return Ok(());
    }
    server_admin_contract::admin_table_sort_field::AdminTableSortField::try_from_key(
        options,
        server_admin_contract::admin_table_sort_key_ref::AdminTableSortKeyRef::from(
            query.sort().as_ref(),
        ),
    )
    .map(drop)
    .map_err(|_error| crate::admin_error::AdminError::Validation)
}
