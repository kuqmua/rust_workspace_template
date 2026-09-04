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
    macro_helpers::generate_validated_tokens::generate_validated_tokens(
        proc_macro2_token_stream_ref,
        crate::parse_generate_pg_table::parse_generate_pg_table,
        crate::build_generate_pg_table::build_generate_pg_table,
        crate::validate_generate_pg_table::validate_generate_pg_table,
        crate::emit_generate_pg_table::emit_generate_pg_table,
        |error| {
            macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
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
            )
        },
    )
}
