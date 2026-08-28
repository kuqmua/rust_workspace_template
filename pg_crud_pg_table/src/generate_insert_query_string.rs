#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub(super) fn generate_insert_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    values: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
    insert_values_fmt: InsertValuesFmt,
) -> PgTableQueryString {
    let wrapper_len = match insert_values_fmt {
        InsertValuesFmt::Raw => constants_usize::ZERO,
        InsertValuesFmt::Wrapped => 2usize,
    };
    let mut query = String::with_capacity(
        34usize
            .saturating_add(table.as_ref().len())
            .saturating_add(cols.as_ref().len())
            .saturating_add(values.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(wrapper_len),
    );
    query.push_str(constants_str::INSERT_INTO);
    query.push_str(table.as_ref());
    query.push_str(constants_str::TEXT);
    query.push_str(cols.as_ref());
    query.push_str(constants_str::VALUES);
    if matches!(insert_values_fmt, InsertValuesFmt::Wrapped) {
        query.push('(');
    }
    query.push_str(values.as_ref());
    if matches!(insert_values_fmt, InsertValuesFmt::Wrapped) {
        query.push(')');
    }
    query.push_str(constants_str::RETURNING);
    query.push_str(cols_to_return.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
