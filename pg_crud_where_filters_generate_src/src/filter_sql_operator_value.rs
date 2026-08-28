pub(crate) const fn filter_sql_operator_value(
    spec: crate::spec::FilterSpec,
) -> crate::spec::FilterSqlOperator {
    spec.sql_operator()
}
