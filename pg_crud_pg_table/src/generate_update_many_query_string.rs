#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub fn generate_update_many_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    elements: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    primary_key_field_name: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    pks: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    cols_to_return: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> Result<
    crate::pg_table_query_string::PgTableQueryString,
    crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError,
> {
    let mut query = String::with_capacity(
        30usize
            .saturating_add(pg_table_name_ref.as_ref().len())
            .saturating_add(elements.as_ref().len())
            .saturating_add(primary_key_field_name.as_ref().len())
            .saturating_add(pks.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(6usize),
    );
    query.push_str(constants_str::UPDATE_ALT);
    query.push_str(pg_table_name_ref.as_ref());
    query.push_str(constants_str::SET);
    query.push_str(elements.as_ref());
    query.push_str(constants_str::WHERE);
    query.push_str(primary_key_field_name.as_ref());
    query.push_str(constants_str::IN);
    query.push_str(pks.as_ref());
    query.push(')');
    query.push_str(constants_str::RETURNING);
    query.push_str(cols_to_return.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
}
