pub(super) fn impl_identifier_token_stream_impl(
    identifier_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        impl #identifier_token_stream {
            #ts
        }
    }
    .into()
}
