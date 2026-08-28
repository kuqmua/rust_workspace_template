pub(crate) use crate::bind_count::BindCount;
pub(super) use crate::filter_spec::FilterSpec;
pub(super) use crate::filter_spec_valid::FilterSpecValid;
pub(super) use crate::filter_sql_operator::FilterSqlOperator;
pub(super) use crate::filter_sql_suffix::FilterSqlSuffix;
pub(crate) use crate::filter_value_shape::FilterValueShape;
#[cfg(test)]
#[allow(clippy::needless_for_each)] // descriptor matrix avoids repository-forbidden for loops
mod tests {
    #[test]
    fn filter_specs_keep_sql_bind_and_value_shape_in_sync() {
        [
            super::FilterSpec::adjacent(),
            super::FilterSpec::before(),
            super::FilterSpec::contains(),
            super::FilterSpec::equality(),
            super::FilterSpec::left_of(),
            super::FilterSpec::overlaps(),
            super::FilterSpec::right_of(),
            super::FilterSpec::text_search(),
            super::FilterSpec::within(),
        ]
        .into_iter()
        .for_each(|spec| {
            assert!(
                crate::filter_spec_contract_is_valid::filter_spec_contract_is_valid(spec).get()
            );
        });
    }
}
