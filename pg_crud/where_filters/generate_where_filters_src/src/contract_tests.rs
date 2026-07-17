#[allow(clippy::single_call_fn)] // validation remains an independently testable typed pipeline stage
pub(super) fn filter_spec_contract_is_valid(
    spec: crate::model::FilterSpec,
) -> crate::model::FilterSpecValid {
    crate::model::FilterSpecValid::from(
        crate::bind::bind_count_matches(spec, crate::bind::FilterPlaceholderCount::one()).get()
            && crate::schema::schema_uses_text_value(spec).get()
                == crate::client::client_uses_text_value(spec).get()
            && crate::schema::schema_uses_text_value(spec).get()
                != crate::sql::filter_sql_suffix(spec).as_ref().is_empty()
            && !crate::sql::filter_sql_operator(spec).as_ref().is_empty(),
    )
}
#[cfg(test)]
#[allow(clippy::needless_for_each)]
mod tests {
    #[test]
    fn sql_bind_schema_and_client_read_one_filter_spec() {
        [
            crate::model::FilterSpec::ADJACENT,
            crate::model::FilterSpec::BEFORE,
            crate::model::FilterSpec::CONTAINS,
            crate::model::FilterSpec::EQUALITY,
            crate::model::FilterSpec::LEFT_OF,
            crate::model::FilterSpec::OVERLAPS,
            crate::model::FilterSpec::RIGHT_OF,
            crate::model::FilterSpec::TEXT_SEARCH,
            crate::model::FilterSpec::WITHIN,
        ]
        .into_iter()
        .for_each(|spec| {
            assert!(
                crate::bind::bind_count_matches(spec, crate::bind::FilterPlaceholderCount::one())
                    .get()
            );
            assert_eq!(
                crate::schema::schema_uses_text_value(spec).get(),
                crate::client::client_uses_text_value(spec).get()
            );
            assert!(!crate::sql::filter_sql_operator(spec).as_ref().is_empty());
        });
    }
}
