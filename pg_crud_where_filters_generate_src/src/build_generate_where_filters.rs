pub fn build_generate_where_filters(
    parsed: crate::source::ParsedGenerateWhereFiltersConfig,
) -> Result<
    crate::source::BuiltGenerateWhereFiltersModel,
    crate::source::GenerateWhereFiltersPipelineError,
> {
    let valid = [
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
    .all(|spec| {
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
    });
    Ok(crate::source::BuiltGenerateWhereFiltersModel {
        config: parsed,
        contract_valid: crate::spec::FilterSpecValid::from(valid),
    })
}
