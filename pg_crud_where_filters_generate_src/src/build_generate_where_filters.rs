pub fn build_generate_where_filters(
    parsed: crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
) -> Result<
    crate::built_generate_where_filters_model::BuiltGenerateWhereFiltersModel,
    crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError,
> {
    let valid = [
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
    Ok(
        crate::built_generate_where_filters_model::BuiltGenerateWhereFiltersModel {
            config: parsed,
            contract_valid: crate::filter_spec_valid::FilterSpecValid::from(valid),
        },
    )
}
