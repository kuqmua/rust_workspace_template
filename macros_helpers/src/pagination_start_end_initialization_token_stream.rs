pub fn pagination_start_end_initialization_token_stream(
    v: &dyn quote::ToTokens,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        let start = #v.pagination.start();
        let end = #v.pagination.end();
    }
    .into()
}
