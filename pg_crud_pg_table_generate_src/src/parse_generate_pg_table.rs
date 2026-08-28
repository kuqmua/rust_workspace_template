pub fn parse_generate_pg_table(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> Result<crate::SynParsedGeneratePgTableInput, crate::GeneratePgTablePipelineError> {
    syn::parse2::<syn::DeriveInput>(input.as_ref().clone())
        .map(crate::SynParsedGeneratePgTableInput::from)
        .map_err(|error| {
            crate::GeneratePgTablePipelineError::Parse(
                crate::SynGeneratePgTablePipelineError::from(error),
            )
        })
}
