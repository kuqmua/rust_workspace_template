pub fn validate_generate_pg_table(
    built: crate::pipeline::SynBuiltGeneratePgTableInput,
) -> Result<
    crate::pipeline::SynValidatedGeneratePgTableInput,
    crate::pipeline::GeneratePgTablePipelineError,
> {
    built
        .0
        .validate()
        .map(crate::pipeline::SynValidatedGeneratePgTableInput::from)
        .map_err(|error| {
            crate::pipeline::GeneratePgTablePipelineError::Validate(
                crate::pipeline::SynGeneratePgTablePipelineError::from(syn::Error::from(error)),
            )
        })
}
