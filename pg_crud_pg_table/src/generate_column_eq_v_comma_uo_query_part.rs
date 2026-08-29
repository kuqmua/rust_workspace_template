#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn generate_column_eq_v_comma_uo_query_part(
    column: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    value: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_part_fragment::PgTableQueryPartFragment {
    let mut query_part = String::with_capacity(
        column
            .as_ref()
            .len()
            .saturating_add(value.as_ref().len())
            .saturating_add(5),
    );
    if std::fmt::Write::write_fmt(&mut query_part, format_args!("{column} = {value},")).is_err() {
        return crate::pg_table_query_part_fragment::PgTableQueryPartFragment::try_from(
            String::default(),
        )
        .unwrap_or_else(crate::pg_table_query_part_fragment::PgTableQueryPartFragment::from);
    }
    crate::pg_table_query_part_fragment::PgTableQueryPartFragment::try_from(query_part)
        .unwrap_or_else(crate::pg_table_query_part_fragment::PgTableQueryPartFragment::from)
}
