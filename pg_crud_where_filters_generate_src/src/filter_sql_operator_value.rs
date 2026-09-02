pub(crate) const fn filter_sql_operator_value(
    filter_spec: crate::filter_spec::FilterSpec,
) -> crate::filter_sql_operator::FilterSqlOperator {
    filter_spec.sql_operator()
}
