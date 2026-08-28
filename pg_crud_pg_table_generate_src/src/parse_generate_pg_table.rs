pub fn parse_generate_pg_table(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> Result<
    crate::pipeline::SynParsedGeneratePgTableInput,
    crate::pipeline::GeneratePgTablePipelineError,
> {
    syn::parse2::<syn::DeriveInput>(input.as_ref().clone())
        .map(crate::pipeline::SynParsedGeneratePgTableInput::from)
        .map_err(|error| {
            crate::pipeline::GeneratePgTablePipelineError::Parse(
                crate::pipeline::SynGeneratePgTablePipelineError::from(error),
            )
        })
}
