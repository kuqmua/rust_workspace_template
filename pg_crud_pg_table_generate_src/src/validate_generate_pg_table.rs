pub fn validate_generate_pg_table(
    syn_built_generate_pg_table_input: crate::syn_built_generate_pg_table_input::SynBuiltGeneratePgTableInput,
) -> Result<
    crate::syn_validated_generate_pg_table_input::SynValidatedGeneratePgTableInput,
    crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError,
> {
    syn_built_generate_pg_table_input
        .into_model()
        .validate()
        .map(crate::syn_validated_generate_pg_table_input::SynValidatedGeneratePgTableInput::from)
        .map_err(|error| {
            crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Validate(
                crate::syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError::from(
                    syn::Error::from(error),
                ),
            )
        })
}
