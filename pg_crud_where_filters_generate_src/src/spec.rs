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
                crate::bind_count_matches::bind_count_matches(
                    spec,
                    crate::filter_placeholder_count::FilterPlaceholderCount::one(),
                )
                .get()
                    && crate::schema_uses_text_value::schema_uses_text_value(spec).get()
                        == crate::client_uses_text_value::client_uses_text_value(spec).get()
                    && crate::schema_uses_text_value::schema_uses_text_value(spec).get()
                        != crate::filter_sql_suffix_value::filter_sql_suffix_value(spec)
                            .as_ref()
                            .is_empty()
                    && !crate::filter_sql_operator_value::filter_sql_operator_value(spec)
                        .as_ref()
                        .is_empty()
            );
        });
    }
}
