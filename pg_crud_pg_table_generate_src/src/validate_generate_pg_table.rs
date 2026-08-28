pub fn validate_generate_pg_table(
    built: crate::SynBuiltGeneratePgTableInput,
) -> Result<crate::SynValidatedGeneratePgTableInput, crate::GeneratePgTablePipelineError> {
    built
        .0
        .validate()
        .map(crate::SynValidatedGeneratePgTableInput::from)
        .map_err(|error| {
            crate::GeneratePgTablePipelineError::Validate(
                crate::SynGeneratePgTablePipelineError::from(syn::Error::from(error)),
            )
        })
}
