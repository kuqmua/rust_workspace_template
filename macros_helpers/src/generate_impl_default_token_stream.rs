pub fn generate_impl_default_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        impl Default for #identifier {
            fn default() -> Self {
                #ts
            }
        }
    }
    .into()
}
