#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

#[must_use]
pub fn add_uo_optimistic_revision_predicate(
    query: crate::pg_table_query_string::PgTableQueryString,
    revision_column: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    expected_revision_query_part: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
) -> crate::pg_table_query_string::PgTableQueryString {
    let query_text = query.to_string();
    let Some((statement, returning)) = query_text.rsplit_once(constants_str::catalog::RETURNING)
    else {
        return query;
    };
    let mut optimistic_query = String::with_capacity(
        query_text
            .len()
            .saturating_add(revision_column.as_ref().len().saturating_mul(2usize))
            .saturating_add(expected_revision_query_part.as_ref().len())
            .saturating_add(9usize),
    );
    optimistic_query.push_str(statement);
    optimistic_query.push_str(constants_str::catalog::AND);
    optimistic_query.push_str(revision_column.as_ref());
    optimistic_query.push_str(constants_str::catalog::TEXT_ALT);
    optimistic_query.push_str(expected_revision_query_part.as_ref());
    optimistic_query.push_str(constants_str::catalog::RETURNING);
    optimistic_query.push_str(returning);
    crate::pg_table_query_string::PgTableQueryString::try_from(optimistic_query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
