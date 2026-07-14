pub fn generate_impl_from_token_stream(
    from_type_token_stream: &dyn quote::ToTokens,
    for_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let v_snake_case = naming::VSnakeCase;
    quote::quote! {
        impl From<#from_type_token_stream> for #for_type_token_stream {
            fn from(#v_snake_case: #from_type_token_stream) -> Self {
                #ts
            }
        }
    }
    .into()
}
