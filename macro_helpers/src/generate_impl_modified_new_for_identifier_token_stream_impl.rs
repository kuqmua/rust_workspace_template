pub(super) fn generate_impl_modified_new_for_identifier_token_stream_impl(
    identifier_token_stream: &dyn quote::ToTokens,
    attr_token_stream: &dyn quote::ToTokens,
    modifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    super::impl_identifier_token_stream_impl::impl_identifier_token_stream_impl(
        identifier_token_stream,
        &super::generate_modified_new_token_stream_impl::generate_modified_new_token_stream_impl(
            attr_token_stream,
            modifier_token_stream,
            parameters_token_stream,
            ts,
        ),
    )
}
