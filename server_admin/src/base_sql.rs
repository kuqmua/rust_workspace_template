#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn base_sql(
    table: server_admin_contract::domain_types::AdminDataTable,
) -> Result<
    (
        crate::domain_types::StdAdminString,
        crate::domain_types::StdAdminString,
    ),
    crate::AdminRepositoryError,
> {
    let spec = table.spec();
    let table_name = table.to_string();
    let mut count = constants_str::SERVER_ADMIN_DATA_COUNT_PREFIX.to_owned();
    count.push_str(table_name.as_str());
    let mut data = spec.columns().get().split(',').enumerate().fold(
        constants_str::SERVER_ADMIN_DATA_SELECT_ARRAY_PREFIX.to_owned(),
        |mut sql, (index, column)| {
            if index > constants_usize::ZERO {
                sql.push_str(constants_str::TEXT_ALT_7);
            }
            sql.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_COLUMN_PREFIX);
            sql.push_str(column);
            sql.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_COLUMN_SUFFIX);
            sql
        },
    );
    data.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_FROM);
    data.push_str(table_name.as_str());
    data.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
    data.push_str(spec.order().get());
    data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR);
    Ok((
        crate::domain_types::StdAdminString::try_from(count)
            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
        crate::domain_types::StdAdminString::try_from(data)
            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
    ))
}
