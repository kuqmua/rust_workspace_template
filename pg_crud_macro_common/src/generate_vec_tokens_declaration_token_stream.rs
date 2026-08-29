pub fn generate_vec_tokens_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {Vec<#type_token_stream>}.into()
}
