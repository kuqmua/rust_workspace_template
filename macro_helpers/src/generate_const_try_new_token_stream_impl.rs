pub fn generate_const_try_new_token_stream_impl(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    super::generate_modified_try_new_token_stream_impl::generate_modified_try_new_token_stream_impl(
        attr_token_stream,
        &quote::quote! {const},
        parameters_token_stream,
        err_type_token_stream,
        ts,
    )
}
