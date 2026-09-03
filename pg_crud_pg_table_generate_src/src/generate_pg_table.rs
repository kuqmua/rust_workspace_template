#[must_use]
#[allow(
    non_snake_case,
    reason = "generate pg table requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]
#[allow(
    unused_variables,
    reason = "generate pg table emits configuration-dependent bindings that are unused in some generated variants"
)]
pub fn generate_pg_table(
    proc_macro2_token_stream_ref: macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef<'_>,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let validated = match crate::parse_generate_pg_table::parse_generate_pg_table(
        proc_macro2_token_stream_ref,
    )
    .and_then(crate::build_generate_pg_table::build_generate_pg_table)
    .and_then(crate::validate_generate_pg_table::validate_generate_pg_table)
    {
        Ok(validated) => validated,
        Err(error) => {
            return macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
                match error {
                    crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Build(
                        pipeline_error,
                    )
                    | crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Parse(
                        pipeline_error,
                    )
                    | crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Validate(
                        pipeline_error,
                    ) => syn::Error::from(pipeline_error).to_compile_error(),
                },
            );
        }
    };
    crate::emit_generate_pg_table::emit_generate_pg_table(validated)
}
