#[cfg(test)]
#[allow(
    clippy::needless_for_each,
    reason = "spec tests uses iterator traversal to comply with the workspace no-for-loop policy"
)]
mod tests {
    #[test]
    fn test_filter_specs_keep_sql_bind_and_value_shape_in_sync() {
        [
            crate::filter_spec::FilterSpec::adjacent(),
            crate::filter_spec::FilterSpec::before(),
            crate::filter_spec::FilterSpec::contains(),
            crate::filter_spec::FilterSpec::equality(),
            crate::filter_spec::FilterSpec::left_of(),
            crate::filter_spec::FilterSpec::overlaps(),
            crate::filter_spec::FilterSpec::right_of(),
            crate::filter_spec::FilterSpec::text_search(),
            crate::filter_spec::FilterSpec::within(),
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
