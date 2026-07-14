pub fn pagination_start_end_initialization_token_stream(
    v: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        let start = #v.pagination.start();
        let end = #v.pagination.end();
    }
    .into()
}
