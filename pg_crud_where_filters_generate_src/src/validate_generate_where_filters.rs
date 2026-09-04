pub fn validate_generate_where_filters(
    built_generate_where_filters_model: crate::built_generate_where_filters_model::BuiltGenerateWhereFiltersModel,
) -> Result<
    crate::validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig,
    crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError,
> {
    if built_generate_where_filters_model.contract_valid().get() {
        Ok(crate::validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig::from(
            built_generate_where_filters_model.config(),
        ))
    } else {
        Err(crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError::InvalidContract)
    }
}
