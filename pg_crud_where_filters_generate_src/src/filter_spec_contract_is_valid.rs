// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // validation remains an independently testable typed pipeline stage
pub(super) fn filter_spec_contract_is_valid(
    spec: crate::spec::FilterSpec,
) -> crate::spec::FilterSpecValid {
    crate::spec::FilterSpecValid::from(
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
                .is_empty(),
    )
}
#[cfg(test)]
#[allow(
    clippy::needless_for_each,
    reason = "the table-driven contract test applies the same invariant to every filter spec"
)]
mod tests {
    #[test]
    fn sql_bind_schema_and_client_read_one_filter_spec() {
        [
            crate::spec::FilterSpec::adjacent(),
            crate::spec::FilterSpec::before(),
            crate::spec::FilterSpec::contains(),
            crate::spec::FilterSpec::equality(),
            crate::spec::FilterSpec::left_of(),
            crate::spec::FilterSpec::overlaps(),
            crate::spec::FilterSpec::right_of(),
            crate::spec::FilterSpec::text_search(),
            crate::spec::FilterSpec::within(),
        ]
        .into_iter()
        .for_each(|spec| {
            assert!(
                crate::bind_count_matches::bind_count_matches(
                    spec,
                    crate::filter_placeholder_count::FilterPlaceholderCount::one()
                )
                .get()
            );
            assert_eq!(
                crate::schema_uses_text_value::schema_uses_text_value(spec).get(),
                crate::client_uses_text_value::client_uses_text_value(spec).get()
            );
            assert!(
                !crate::filter_sql_operator_value::filter_sql_operator_value(spec)
                    .as_ref()
                    .is_empty()
            );
        });
    }
}
