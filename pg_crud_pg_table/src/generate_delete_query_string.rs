#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub(super) fn generate_delete_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    pg_table_sql_fragment_ref: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    option: Option<crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>>,
) -> crate::pg_table_query_string::PgTableQueryString {
    let where_len = option.map_or_else(
        || 12usize.saturating_add(pg_table_sql_fragment_ref.as_ref().len()),
        |v| v.as_ref().len(),
    );
    let mut query = String::with_capacity(
        24usize
            .saturating_add(pg_table_name_ref.as_ref().len())
            .saturating_add(where_len)
            .saturating_add(pg_table_sql_fragment_ref.as_ref().len()),
    );
    query.push_str(constants_str::DELETE_FROM);
    query.push_str(pg_table_name_ref.as_ref());
    query.push(' ');
    if let Some(v) = option {
        query.push_str(v.as_ref());
    } else {
        query.push_str(constants_str::WHERE_ALT);
        query.push_str(pg_table_sql_fragment_ref.as_ref());
        query.push_str(constants_str::DOLLAR_1);
    }
    query.push_str(constants_str::RETURNING);
    query.push_str(pg_table_sql_fragment_ref.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
