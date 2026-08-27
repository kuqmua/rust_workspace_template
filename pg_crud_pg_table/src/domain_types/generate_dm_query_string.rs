#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn generate_dm_query_string(
    table: PgTableNameRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_delete_query_string(table, primary_key_field_name, Some(where_string))
}
