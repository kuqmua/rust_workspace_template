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
    .all(|spec| crate::filter_spec_contract_is_valid::filter_spec_contract_is_valid(spec).get());
    Ok(crate::source::BuiltGenerateWhereFiltersModel {
        config: parsed,
        contract_valid: crate::spec::FilterSpecValid::from(valid),
    })
}
