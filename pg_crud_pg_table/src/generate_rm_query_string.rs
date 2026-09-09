#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn generate_rm_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    select_string: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    where_string: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_string::PgTableQueryString {
    let mut query = String::with_capacity(
        13usize
            .saturating_add(select_string.as_ref().len())
            .saturating_add(pg_table_name_ref.as_ref().len())
            .saturating_add(where_string.as_ref().len())
            .saturating_add(constants_usize::ONE),
    );
    query.push_str(constants_str::SELECT_ALT);
    query.push_str(select_string.as_ref());
    query.push_str(constants_str::FROM_ALT);
    query.push_str(pg_table_name_ref.as_ref());
    query.push(' ');
    query.push_str(where_string.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
