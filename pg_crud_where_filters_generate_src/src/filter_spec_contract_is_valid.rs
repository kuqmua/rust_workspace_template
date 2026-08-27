#[allow(clippy::single_call_fn)] // validation remains an independently testable typed pipeline stage
pub(super) fn filter_spec_contract_is_valid(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSpecValid {
    crate::domain_types::spec::FilterSpecValid::from(
        crate::domain_types::bind_count_matches::bind_count_matches(
            spec,
            crate::domain_types::filter_placeholder_count::FilterPlaceholderCount::one(),
        )
        .get()
            && crate::domain_types::schema::schema_uses_text_value::schema_uses_text_value(spec)
                .get()
                == crate::domain_types::client::client_uses_text_value::client_uses_text_value(
                    spec,
                )
                .get()
            && crate::domain_types::schema::schema_uses_text_value::schema_uses_text_value(spec)
                .get()
                != crate::domain_types::sql::filter_sql_suffix::filter_sql_suffix(spec)
                    .as_ref()
                    .is_empty()
            && !crate::domain_types::sql::filter_sql_operator::filter_sql_operator(spec)
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
            crate::domain_types::spec::FilterSpec::adjacent(),
            crate::domain_types::spec::FilterSpec::before(),
            crate::domain_types::spec::FilterSpec::contains(),
            crate::domain_types::spec::FilterSpec::equality(),
            crate::domain_types::spec::FilterSpec::left_of(),
            crate::domain_types::spec::FilterSpec::overlaps(),
            crate::domain_types::spec::FilterSpec::right_of(),
            crate::domain_types::spec::FilterSpec::text_search(),
            crate::domain_types::spec::FilterSpec::within(),
        ]
        .into_iter()
        .for_each(|spec| {
            assert!(
                crate::domain_types::bind_count_matches::bind_count_matches(
                    spec,
                    crate::domain_types::filter_placeholder_count::FilterPlaceholderCount::one()
                )
                .get()
            );
            assert_eq!(
                crate::domain_types::schema::schema_uses_text_value::schema_uses_text_value(spec)
                    .get(),
                crate::domain_types::client::client_uses_text_value::client_uses_text_value(spec)
                    .get()
            );
            assert!(
                !crate::domain_types::sql::filter_sql_operator::filter_sql_operator(spec)
                    .as_ref()
                    .is_empty()
            );
        });
    }
}
