use super::super::*;

#[must_use]
pub fn generate_pg_types(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    match parse_generate_pg_types(input)
        .and_then(build_generate_pg_types)
        .and_then(validate_generate_pg_types)
    {
        Ok(validated) => emit_generate_pg_types(validated),
        Err(error) => {
            let message = format!("failed to parse GeneratePgTypesConfig: {error}");
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
