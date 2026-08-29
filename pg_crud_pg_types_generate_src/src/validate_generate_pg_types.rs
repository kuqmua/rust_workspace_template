pub fn validate_generate_pg_types(
    built: crate::built_generate_pg_types_model::BuiltGeneratePgTypesModel,
) -> Result<
    crate::validated_generate_pg_types_config::ValidatedGeneratePgTypesConfig,
    crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError,
> {
    Ok(
        crate::validated_generate_pg_types_config::ValidatedGeneratePgTypesConfig {
            config: built.config,
            entry_count: built.entry_count,
        },
    )
}
