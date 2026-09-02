#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub(super) fn generate_insert_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    cols: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    values: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    cols_to_return: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    insert_values_fmt: crate::insert_values_fmt::InsertValuesFmt,
) -> crate::pg_table_query_string::PgTableQueryString {
    let wrapper_len = match insert_values_fmt {
        crate::insert_values_fmt::InsertValuesFmt::Raw => constants_usize::ZERO,
        crate::insert_values_fmt::InsertValuesFmt::Wrapped => 2usize,
    };
    let mut query = String::with_capacity(
        34usize
            .saturating_add(pg_table_name_ref.as_ref().len())
            .saturating_add(cols.as_ref().len())
            .saturating_add(values.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(wrapper_len),
    );
    query.push_str(constants_str::INSERT_INTO);
    query.push_str(pg_table_name_ref.as_ref());
    query.push_str(constants_str::TEXT);
    query.push_str(cols.as_ref());
    query.push_str(constants_str::VALUES);
    if matches!(
        insert_values_fmt,
        crate::insert_values_fmt::InsertValuesFmt::Wrapped
    ) {
        query.push('(');
    }
    query.push_str(values.as_ref());
    if matches!(
        insert_values_fmt,
        crate::insert_values_fmt::InsertValuesFmt::Wrapped
    ) {
        query.push(')');
    }
    query.push_str(constants_str::RETURNING);
    query.push_str(cols_to_return.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
