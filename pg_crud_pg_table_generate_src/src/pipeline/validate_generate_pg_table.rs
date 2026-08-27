pub fn validate_generate_pg_table(
    built: super::SynBuiltGeneratePgTableInput,
) -> Result<super::SynValidatedGeneratePgTableInput, super::GeneratePgTablePipelineError> {
    built
        .0
        .validate()
        .map(super::SynValidatedGeneratePgTableInput::from)
        .map_err(|error| {
            super::GeneratePgTablePipelineError::Validate(
                super::SynGeneratePgTablePipelineError::from(syn::Error::from(error)),
            )
        })
}
