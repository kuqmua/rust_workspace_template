#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn generate_uo_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    cols: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    primary_key_field_name: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    primary_key_query_part: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    cols_to_return: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_string::PgTableQueryString {
    crate::generate_update_query_string::generate_update_query_string(
        pg_table_name_ref,
        cols,
        primary_key_field_name,
        primary_key_query_part,
        cols_to_return,
        crate::update_selector_fmt::UpdateSelectorFmt::Eq,
    )
}
