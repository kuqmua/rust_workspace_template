#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub(super) fn generate_select_query_string(
    table: crate::pg_table_name_ref::PgTableNameRef<'_>,
    select_string: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    where_string: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    select_where_fmt: crate::select_where_fmt::SelectWhereFmt,
) -> crate::pg_table_query_string::PgTableQueryString {
    let where_len = match select_where_fmt {
        crate::select_where_fmt::SelectWhereFmt::Plain => constants_usize::ONE,
        crate::select_where_fmt::SelectWhereFmt::Where => 7usize,
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
        crate::select_where_fmt::SelectWhereFmt::Plain => query.push(' '),
        crate::select_where_fmt::SelectWhereFmt::Where => {
            query.push_str(constants_str::WHERE);
        }
    }
    query.push_str(where_string.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
