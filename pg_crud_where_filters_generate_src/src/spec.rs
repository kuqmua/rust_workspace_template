#[path = "bind_count.rs"]
mod bind_count;
#[path = "filter_spec.rs"]
mod filter_spec;
#[path = "filter_spec_valid.rs"]
mod filter_spec_valid;
#[path = "filter_sql_operator.rs"]
mod filter_sql_operator;
#[path = "filter_sql_suffix.rs"]
mod filter_sql_suffix;
#[path = "filter_value_shape.rs"]
mod filter_value_shape;

use bind_count::BindCount;
pub(super) use filter_spec::FilterSpec;
pub(super) use filter_spec_valid::FilterSpecValid;
pub(super) use filter_sql_operator::FilterSqlOperator;
pub(super) use filter_sql_suffix::FilterSqlSuffix;
use filter_value_shape::FilterValueShape;
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
                crate::domain_types::filter_spec_contract_is_valid::filter_spec_contract_is_valid(
                    spec
                )
                .get()
            );
        });
    }
}
