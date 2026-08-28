pub fn generate_optional_type_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {Option<#type_token_stream>}.into()
}
