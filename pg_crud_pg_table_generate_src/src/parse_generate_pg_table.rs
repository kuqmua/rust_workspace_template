pub fn parse_generate_pg_table(
    proc_macro2_token_stream_ref: macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef<'_>,
) -> Result<
    crate::syn_parsed_generate_pg_table_input::SynParsedGeneratePgTableInput,
    crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError,
> {
    syn::parse2::<syn::DeriveInput>(proc_macro2_token_stream_ref.as_ref().clone())
        .map(crate::syn_parsed_generate_pg_table_input::SynParsedGeneratePgTableInput::from)
        .map_err(|error| {
            crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Parse(
                crate::syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError::from(
                    error,
                ),
            )
        })
}
