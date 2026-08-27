#[must_use]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn generate_pg_table(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let validated = match crate::domain_types::pipeline::parse_generate_pg_table(input)
        .and_then(crate::domain_types::pipeline::build_generate_pg_table)
        .and_then(crate::domain_types::pipeline::validate_generate_pg_table)
    {
        Ok(validated) => validated,
        Err(error) => {
            return macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
                match error {
                    crate::domain_types::pipeline::GeneratePgTablePipelineError::Build(
                        pipeline_error,
                    )
                    | crate::domain_types::pipeline::GeneratePgTablePipelineError::Parse(
                        pipeline_error,
                    )
                    | crate::domain_types::pipeline::GeneratePgTablePipelineError::Validate(
                        pipeline_error,
                    ) => syn::Error::from(pipeline_error).to_compile_error(),
                },
            );
        }
    };
    super::emit_generate_pg_table::emit_generate_pg_table(validated)
}
