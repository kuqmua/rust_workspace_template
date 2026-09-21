pub(crate) fn admin_identifier_filter_query(
    admin_csr_query: &crate::admin_csr_query::AdminCsrQuery,
) -> Result<
    server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    if let Some(access_session_id) = admin_csr_query.access_session_id() {
        return Ok(
            server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
                Some(
                    server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                        String::from(constants_str::SQL_NAMES_ID),
                    )
                    .map_err(|error| {
                        server_admin_contract::admin_text::AdminText::try_from(error.to_string())
                            .map_or(
                                crate::admin_table_load_error::AdminTableLoadError::Query,
                                crate::admin_table_load_error::AdminTableLoadError::QuerySource,
                            )
                    })?,
                ),
                Some(frontend_contract::filter_operation::FilterOperation::Eq),
                Some(
                    server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                        access_session_id.to_string(),
                    )
                    .map_err(|error| {
                        server_admin_contract::admin_text::AdminText::try_from(error.to_string())
                            .map_or(
                                crate::admin_table_load_error::AdminTableLoadError::Query,
                                crate::admin_table_load_error::AdminTableLoadError::QuerySource,
                            )
                    })?,
                ),
                None,
            ),
        );
    }
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
