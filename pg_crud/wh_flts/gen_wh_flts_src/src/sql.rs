#![allow(
    clippy::single_call_fn,
    reason = "the SQL emitter boundary is intentionally isolated from descriptor and source assembly"
)]
pub(super) const fn filter_sql_operator(
    spec: crate::model::FilterSpec,
) -> crate::model::FilterSqlOperator {
    spec.sql_operator()
}
pub(super) const fn filter_sql_suffix(
    spec: crate::model::FilterSpec,
) -> crate::model::FilterSqlSuffix {
    spec.sql_suffix()
}
