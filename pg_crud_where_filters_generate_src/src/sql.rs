#![allow(
    clippy::single_call_fn,
    reason = "the SQL emitter boundary is intentionally isolated from descriptor and source assembly"
)]
pub(super) const fn filter_sql_operator(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSqlOperator {
    spec.sql_operator()
}
pub(super) const fn filter_sql_suffix(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSqlSuffix {
    spec.sql_suffix()
}
