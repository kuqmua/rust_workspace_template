#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

//todo extra param for cols_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
#[must_use]
pub fn generate_um_query_string(
    table: PgTableNameRef<'_>,
    els: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    pks: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_update_query_string(
        table,
        els,
        primary_key_field_name,
        pks,
        cols_to_return,
        UpdateSelectorFmt::InList,
    )
}
