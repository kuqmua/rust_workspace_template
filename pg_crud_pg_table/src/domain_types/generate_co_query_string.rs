#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn generate_co_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    values: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_insert_query_string(
        table,
        cols,
        values,
        cols_to_return,
        InsertValuesFmt::Wrapped,
    )
}
