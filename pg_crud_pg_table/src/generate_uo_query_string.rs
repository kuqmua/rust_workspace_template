#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

//todo extra param for cols_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
#[must_use]
pub fn generate_uo_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    primary_key_query_part: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_update_query_string(
        table,
        cols,
        primary_key_field_name,
        primary_key_query_part,
        cols_to_return,
        UpdateSelectorFmt::Eq,
    )
}
