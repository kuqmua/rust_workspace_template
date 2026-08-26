#![allow(clippy::single_call_fn)] // SQL suffix projection has one binding owner

pub(in crate::domain_types) const fn filter_sql_suffix(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSqlSuffix {
    spec.sql_suffix()
}
