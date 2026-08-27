pub fn build_generate_where_filters(
    parsed: super::ParsedGenerateWhereFiltersConfig,
) -> Result<super::BuiltGenerateWhereFiltersModel, super::GenerateWhereFiltersPipelineError> {
    let valid = [
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
    .all(|spec| {
        crate::domain_types::filter_spec_contract_is_valid::filter_spec_contract_is_valid(spec)
            .get()
    });
    Ok(super::BuiltGenerateWhereFiltersModel {
        config: parsed,
        contract_valid: crate::domain_types::spec::FilterSpecValid::from(valid),
    })
}
