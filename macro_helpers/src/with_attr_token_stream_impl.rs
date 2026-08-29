pub(super) fn with_attr_token_stream_impl(
    attr_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        #attr_token_stream
        #ts
    }
    .into()
}
