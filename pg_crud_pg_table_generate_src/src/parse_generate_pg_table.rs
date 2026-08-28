pub fn parse_generate_pg_table(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> Result<super::SynParsedGeneratePgTableInput, super::GeneratePgTablePipelineError> {
    syn::parse2::<syn::DeriveInput>(input.as_ref().clone())
        .map(super::SynParsedGeneratePgTableInput::from)
        .map_err(|error| {
            super::GeneratePgTablePipelineError::Parse(
                super::SynGeneratePgTablePipelineError::from(error),
            )
        })
}
