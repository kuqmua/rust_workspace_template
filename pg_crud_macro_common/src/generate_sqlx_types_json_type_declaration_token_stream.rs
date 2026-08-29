pub fn generate_sqlx_types_json_type_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {sqlx::types::Json<#type_token_stream>}.into()
}
