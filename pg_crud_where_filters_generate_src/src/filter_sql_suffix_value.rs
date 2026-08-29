pub(crate) const fn filter_sql_suffix_value(
    spec: crate::filter_spec::FilterSpec,
) -> crate::filter_sql_suffix::FilterSqlSuffix {
    spec.sql_suffix()
}
