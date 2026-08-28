pub fn validate_generate_where_filters(
    built: crate::source::BuiltGenerateWhereFiltersModel,
) -> Result<
    crate::source::ValidatedGenerateWhereFiltersConfig,
    crate::source::GenerateWhereFiltersPipelineError,
> {
    if built.contract_valid.get() {
        Ok(crate::source::ValidatedGenerateWhereFiltersConfig::from(
            built.config,
        ))
    } else {
        Err(crate::source::GenerateWhereFiltersPipelineError::InvalidContract)
    }
}
