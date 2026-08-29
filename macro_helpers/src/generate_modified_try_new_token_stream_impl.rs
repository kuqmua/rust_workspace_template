pub(super) fn generate_modified_try_new_token_stream_impl(
    attr_token_stream: &dyn quote::ToTokens,
    modifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let try_new_token_stream =
        crate::generate_try_new_token_stream_impl::generate_try_new_token_stream_impl(
            &proc_macro2::TokenStream::new(),
            parameters_token_stream,
            err_type_token_stream,
            ts,
        );
    super::with_attr_token_stream_impl::with_attr_token_stream_impl(
        attr_token_stream,
        &quote::quote! {#modifier_token_stream #try_new_token_stream},
    )
}
