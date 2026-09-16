pub(crate) fn admin_identifier_filter_query(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let operation = admin_csr_query
        .filter_operation()
        .map(|operation| match operation.as_ref() {
            constants_str::ADMIN_FILTER_OPERATION_EQ => {
                Ok(frontend_contract::filter_operation::FilterOperation::Eq)
            }
            constants_str::ADMIN_FILTER_OPERATION_GREATER_THAN => {
                Ok(frontend_contract::filter_operation::FilterOperation::GreaterThan)
            }
            constants_str::ADMIN_FILTER_OPERATION_BETWEEN => {
                Ok(frontend_contract::filter_operation::FilterOperation::Between)
            }
            constants_str::ADMIN_FILTER_OPERATION_IN => {
                Ok(frontend_contract::filter_operation::FilterOperation::In)
            }
            _ => Err(crate::admin_table_load_error::AdminTableLoadError::Query),
        })
        .transpose()?;
    Ok(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            admin_csr_query.filter_field().cloned(),
            operation,
            admin_csr_query.filter_value().cloned(),
            admin_csr_query.filter_end().cloned(),
        ),
    )
}
