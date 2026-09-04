#[must_use]
pub fn generate_pg_types_tokens(
    proc_macro2_token_stream_ref: macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef<'_>,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    macro_helpers::generate_validated_tokens::generate_validated_tokens(
        proc_macro2_token_stream_ref,
        crate::parse_generate_pg_types::parse_generate_pg_types,
        crate::build_generate_pg_types::build_generate_pg_types,
        crate::validate_generate_pg_types::validate_generate_pg_types,
        crate::emit_generate_pg_types::emit_generate_pg_types,
        |error| {
            let message = format!("failed to parse GeneratePgTypesConfig: {error}");
            macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        },
    )
}
