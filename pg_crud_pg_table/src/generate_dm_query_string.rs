#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn generate_dm_query_string(
    table: crate::pg_table_name_ref::PgTableNameRef<'_>,
    where_string: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    primary_key_field_name: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_string::PgTableQueryString {
    crate::generate_delete_query_string::generate_delete_query_string(
        table,
        primary_key_field_name,
        Some(where_string),
    )
}
