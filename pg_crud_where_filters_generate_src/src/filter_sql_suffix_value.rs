pub(crate) const fn filter_sql_suffix_value(
    filter_spec: crate::filter_spec::FilterSpec,
) -> crate::filter_sql_suffix::FilterSqlSuffix {
    filter_spec.sql_suffix()
}
