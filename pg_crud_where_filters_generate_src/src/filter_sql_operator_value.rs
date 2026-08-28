#![allow(
    clippy::single_call_fn,
    reason = "the SQL emitter boundary is intentionally isolated from descriptor and source assembly"
)]
pub(in crate::domain_types) const fn filter_sql_operator_value(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSqlOperator {
    spec.sql_operator()
}
