#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub fn generate_when_column_id_then_value_update_many_query_part(
    column: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    id: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    value: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> Result<
    crate::pg_table_query_part_fragment::PgTableQueryPartFragment,
    crate::pg_table_string_wrapper_try_from_string_error::PgTableStringWrapperTryFromStringError,
> {
    crate::pg_table_query_part_fragment::PgTableQueryPartFragment::try_from(format!(
        "when {column} = {id} then {value} "
    ))
}
