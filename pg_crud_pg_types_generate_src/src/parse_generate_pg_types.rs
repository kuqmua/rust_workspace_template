pub fn parse_generate_pg_types(
    input: macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef<'_>,
) -> Result<
    crate::parsed_generate_pg_types_config::ParsedGeneratePgTypesConfig,
    crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError,
> {
    serde_json::from_str::<crate::generate_pg_types_config::GeneratePgTypesConfig>(
        &input.as_ref().to_string(),
    )
    .map(crate::parsed_generate_pg_types_config::ParsedGeneratePgTypesConfig)
    .map_err(|error| {
        crate::generate_pg_types_pipeline_error::GeneratePgTypesPipelineError::Parse(
            crate::serde_json_generate_pg_types_error::SerdeJsonGeneratePgTypesError::from(error),
        )
    })
}
