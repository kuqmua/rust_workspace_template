#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn generate_rm_query_string(
    table: PgTableNameRef<'_>,
    select_string: PgTableSqlFragmentRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_select_query_string(table, select_string, where_string, SelectWhereFmt::Plain)
}
