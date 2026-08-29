#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn generate_co_query_string(
    table: crate::pg_table_name_ref::PgTableNameRef<'_>,
    cols: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    values: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    cols_to_return: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_string::PgTableQueryString {
    crate::generate_insert_query_string::generate_insert_query_string(
        table,
        cols,
        values,
        cols_to_return,
        crate::insert_values_fmt::InsertValuesFmt::Wrapped,
    )
}
