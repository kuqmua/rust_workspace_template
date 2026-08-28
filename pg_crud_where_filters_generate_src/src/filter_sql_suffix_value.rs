pub(crate) const fn filter_sql_suffix_value(
    spec: crate::spec::FilterSpec,
) -> crate::spec::FilterSqlSuffix {
    spec.sql_suffix()
}
