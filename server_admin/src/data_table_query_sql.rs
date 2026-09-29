#[allow(
    clippy::single_call_fn,
    reason = "the pure table SQL builder has direct unit coverage for search, sort, and filter placeholder ordering"
)]
pub(crate) fn data_table_query_sql(
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    filter_fragment: Option<&pg_crud_common::query_part_fragment::QueryPartFragment>,
    query_part_increment: pg_crud_common::query_part_increment::QueryPartIncrement,
) -> Result<
    (
        server_admin_core::std_admin_string::StdAdminString,
        server_admin_core::std_admin_string::StdAdminString,
    ),
    crate::admin_repository_error::AdminRepositoryError,
> {
    let spec = admin_data_table.spec();
    let table_name = admin_data_table.to_string();
    let mut base_count_sql = constants_str::SERVER_ADMIN_DATA_COUNT_PREFIX.to_owned();
    base_count_sql.push_str(table_name.as_str());
    let mut base_sql = spec.columns().get().split(',').enumerate().fold(
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
    base_sql.push_str(constants_str::SERVER_ADMIN_DATA_SELECT_FROM);
    base_sql.push_str(table_name.as_str());
    base_sql.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
    base_sql.push_str(spec.order().get());
    base_sql.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR);
    let (data_prefix, ordered_suffix) = base_sql
        .as_str()
        .split_once(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR)
        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
    let default_order = ordered_suffix
        .strip_suffix(constants_str::SERVER_ADMIN_FILTER_LIMIT_SEPARATOR)
        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
    let sort_key = admin_table_query.sort().as_ref();
    let direction = admin_table_query.direction();
    if sort_key.is_empty()
        && direction == server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
    {
        return Err(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue);
    }
    let order = if sort_key.is_empty() {
        default_order.to_owned()
    } else {
        let column = spec
            .columns()
            .get()
            .split(',')
            .find(|column| *column == sort_key)
            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let direction_sql = match direction {
            server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending => {
                constants_str::SERVER_ADMIN_DATA_SORT_ASC
            }
            server_admin_contract::admin_sort_direction::AdminSortDirection::Descending => {
                constants_str::SERVER_ADMIN_DATA_SORT_DESC
            }
        };
        let mut selected_order = column.to_owned();
        selected_order.push_str(direction_sql);
        if column != constants_str::SQL_NAMES_ID {
            selected_order.push_str(constants_str::SERVER_ADMIN_DATA_SORT_TIE_SEPARATOR);
            selected_order.push_str(direction_sql);
        }
        selected_order
    };
    let mut count_sql = base_count_sql;
    let mut data_sql = data_prefix.to_owned();
    if let Some(fragment) = filter_fragment {
        count_sql.push(' ');
        count_sql.push_str(fragment.as_ref());
        data_sql.push(' ');
        data_sql.push_str(fragment.as_ref());
    }
    let search = admin_table_query.search().as_ref();
    let search_present = !search.is_empty();
    if search_present {
        let search_index = query_part_increment
            .get()
            .checked_add(1u64)
            .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
        let search_index_text = search_index.to_string();
        let mut search_clause = String::new();
        search_clause.push_str(if filter_fragment.is_some() {
            constants_str::AND
        } else {
            constants_str::WHERE
        });
        search_clause.push('(');
        search_clause = spec.columns().get().split(',').enumerate().fold(
            search_clause,
            |mut clause, (index, column)| {
                if index > constants_usize::ZERO {
                    clause.push_str(constants_str::SERVER_ADMIN_DATA_SEARCH_OR_SEPARATOR);
                }
                clause.push('(');
                clause.push_str(column);
                clause.push_str(constants_str::SERVER_ADMIN_DATA_SEARCH_MATCH_PREFIX);
                clause.push_str(search_index_text.as_str());
                clause.push_str(constants_str::SERVER_ADMIN_DATA_SEARCH_MATCH_SUFFIX);
                clause
            },
        );
        search_clause.push(')');
        count_sql.push_str(search_clause.as_str());
        data_sql.push_str(search_clause.as_str());
    }
    let limit_index = query_part_increment
        .get()
        .checked_add(if search_present { 2u64 } else { 1u64 })
        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
    let offset_index = limit_index
        .checked_add(1u64)
        .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
    data_sql.push_str(constants_str::SERVER_ADMIN_FILTER_ORDER_BY_SEPARATOR);
    data_sql.push_str(order.as_str());
    data_sql.push_str(constants_str::SERVER_ADMIN_FILTER_LIMIT_PREFIX);
    data_sql.push_str(limit_index.to_string().as_str());
    data_sql.push_str(constants_str::SERVER_ADMIN_FILTER_OFFSET_PREFIX);
    data_sql.push_str(offset_index.to_string().as_str());
    Ok((
        server_admin_core::std_admin_string::StdAdminString::try_from(count_sql).map_err(
            |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
        )?,
        server_admin_core::std_admin_string::StdAdminString::try_from(data_sql).map_err(
            |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
        )?,
    ))
}
