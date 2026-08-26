pub fn generate_impl_pub_const_try_new_for_identifier_token_stream_impl(
    attr_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    super::generate_impl_modified_try_new_for_identifier_token_stream_impl::generate_impl_modified_try_new_for_identifier_token_stream_impl(
        attr_token_stream,
        identifier_token_stream,
        &quote::quote! { pub const },
        parameters_token_stream,
        err_type_token_stream,
        ts,
    )
}
