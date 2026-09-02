pub fn validate_generate_pg_types(
    built_generate_pg_types_model: crate::built_generate_pg_types_model::BuiltGeneratePgTypesModel,
) -> Result<
    crate::validated_generate_pg_types_config::ValidatedGeneratePgTypesConfig,
    crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError,
> {
    let (config, entry_count) = built_generate_pg_types_model.into_parts();
    Ok(
        crate::validated_generate_pg_types_config::ValidatedGeneratePgTypesConfig::new(
            config,
            entry_count,
        ),
    )
}
