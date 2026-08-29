pub(crate) const fn filter_sql_operator_value(
    spec: crate::filter_spec::FilterSpec,
) -> crate::filter_sql_operator::FilterSqlOperator {
    spec.sql_operator()
}
