#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub(super) fn generate_select_query_string(
    table: PgTableNameRef<'_>,
    select_string: PgTableSqlFragmentRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
    select_where_fmt: SelectWhereFmt,
) -> PgTableQueryString {
    let where_len = match select_where_fmt {
        SelectWhereFmt::Plain => constants_usize::ONE,
        SelectWhereFmt::Where => 7usize,
    };
    let mut query = String::with_capacity(
        13usize
            .saturating_add(select_string.as_ref().len())
            .saturating_add(table.as_ref().len())
            .saturating_add(where_string.as_ref().len())
            .saturating_add(where_len),
    );
    query.push_str(constants_str::SELECT_ALT);
    query.push_str(select_string.as_ref());
    query.push_str(constants_str::FROM_ALT);
    query.push_str(table.as_ref());
    match select_where_fmt {
        SelectWhereFmt::Plain => query.push(' '),
        SelectWhereFmt::Where => query.push_str(constants_str::WHERE),
    }
    query.push_str(where_string.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
