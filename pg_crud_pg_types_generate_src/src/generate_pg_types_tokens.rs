#[must_use]
pub fn generate_pg_types_tokens(
    proc_macro2_token_stream_ref: macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef<'_>,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    match crate::parse_generate_pg_types::parse_generate_pg_types(proc_macro2_token_stream_ref)
        .and_then(crate::build_generate_pg_types::build_generate_pg_types)
        .and_then(crate::validate_generate_pg_types::validate_generate_pg_types)
    {
        Ok(validated) => crate::emit_generate_pg_types::emit_generate_pg_types(validated),
        Err(error) => {
            let message = format!("failed to parse GeneratePgTypesConfig: {error}");
            macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
