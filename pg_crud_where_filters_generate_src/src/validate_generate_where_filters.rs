pub fn validate_generate_where_filters(
    built_generate_where_filters_model: crate::built_generate_where_filters_model::BuiltGenerateWhereFiltersModel,
) -> Result<
    crate::validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig,
    crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError,
> {
    let (config, contract_valid) = built_generate_where_filters_model.into_parts();
    if contract_valid.get() {
        Ok(crate::validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig::from(
            config,
        ))
    } else {
        Err(crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError::InvalidContract)
    }
}
