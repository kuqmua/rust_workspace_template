#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn generate_cm_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    cols: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    values: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    cols_to_return: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_string::PgTableQueryString {
    let mut query = String::with_capacity(
        34usize
            .saturating_add(pg_table_name_ref.as_ref().len())
            .saturating_add(cols.as_ref().len())
            .saturating_add(values.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len()),
    );
    query.push_str(constants_str::INSERT_INTO);
    query.push_str(pg_table_name_ref.as_ref());
    query.push_str(constants_str::TEXT);
    query.push_str(cols.as_ref());
    query.push_str(constants_str::VALUES);
    query.push_str(values.as_ref());
    query.push_str(constants_str::RETURNING);
    query.push_str(cols_to_return.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
