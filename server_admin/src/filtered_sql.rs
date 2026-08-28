pub(super) fn filtered_sql(
    count_sql: crate::domain_types::StdAdminStrRef<'_>,
    data_sql: crate::domain_types::StdAdminStrRef<'_>,
    fragment: &pg_crud_common::domain_types::QueryPartFragment,
    bind_count: pg_crud_common::domain_types::QueryPartIncrement,
) -> Result<
    (
        crate::domain_types::StdAdminString,
        crate::domain_types::StdAdminString,
    ),
    super::AdminRepositoryError,
> {
    let mut filtered_count = count_sql.get().to_owned();
    filtered_count.push(' ');
    filtered_count.push_str(fragment.as_ref());
    let (data_prefix, ordered_suffix) = data_sql
        .get()
        .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
    let order = ordered_suffix
        .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
        .ok_or(super::AdminRepositoryError::InvalidStoredValue)?;
    let limit_index = bind_count.get().saturating_add(1u64);
    let offset_index = limit_index.saturating_add(1u64);
    let mut filtered_data = data_prefix.to_owned();
    filtered_data.push(' ');
    filtered_data.push_str(fragment.as_ref());
    filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
    filtered_data.push_str(order);
    filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX);
    filtered_data.push_str(limit_index.to_string().as_str());
    filtered_data.push_str(constants_str::SERVER_ADMIN_FILTER_OFFSET_PREFIX);
    filtered_data.push_str(offset_index.to_string().as_str());
    let count = crate::domain_types::StdAdminString::try_from(filtered_count)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    let data = crate::domain_types::StdAdminString::try_from(filtered_data)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    Ok((count, data))
}
