pub(crate) fn validate_table_sort(
    query: &server_admin_contract::domain_types::AdminTableQuery,
    options: &[server_admin_contract::domain_types::AdminTableSortField],
) -> Result<(), crate::AdminError> {
    if query.sort().as_ref().is_empty() {
        return Ok(());
    }
    server_admin_contract::domain_types::AdminTableSortField::try_from_key(
        options,
        server_admin_contract::domain_types::AdminTableSortKeyRef::from(query.sort().as_ref()),
    )
    .map(drop)
    .map_err(|_error| crate::AdminError::Validation)
}
