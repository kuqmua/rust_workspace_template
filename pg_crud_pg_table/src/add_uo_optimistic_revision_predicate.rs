#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn add_uo_optimistic_revision_predicate(
    query: PgTableQueryString,
    revision_column: PgTableSqlFragmentRef<'_>,
    expected_revision_query_part: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    let query_text = query.to_string();
    let Some((statement, returning)) = query_text.rsplit_once(constants_str::RETURNING) else {
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
    optimistic_query.push_str(constants_str::AND);
    optimistic_query.push_str(revision_column.as_ref());
    optimistic_query.push_str(constants_str::TEXT_ALT);
    optimistic_query.push_str(expected_revision_query_part.as_ref());
    optimistic_query.push_str(constants_str::RETURNING);
    optimistic_query.push_str(returning);
    PgTableQueryString::try_from(optimistic_query).unwrap_or_else(PgTableQueryString::from)
}
