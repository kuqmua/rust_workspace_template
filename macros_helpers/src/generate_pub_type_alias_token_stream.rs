pub fn generate_pub_type_alias_token_stream(
    alias_type_name_token_stream: &dyn quote::ToTokens,
    alias_actual_type_name_token_stream: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {pub type #alias_type_name_token_stream = #alias_actual_type_name_token_stream;}
        .into()
}
