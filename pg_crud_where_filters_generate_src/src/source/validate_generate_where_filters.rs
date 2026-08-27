pub fn validate_generate_where_filters(
    built: super::BuiltGenerateWhereFiltersModel,
) -> Result<super::ValidatedGenerateWhereFiltersConfig, super::GenerateWhereFiltersPipelineError> {
    if built.contract_valid.get() {
        Ok(super::ValidatedGenerateWhereFiltersConfig::from(
            built.config,
        ))
    } else {
        Err(super::GenerateWhereFiltersPipelineError::InvalidContract)
    }
}
