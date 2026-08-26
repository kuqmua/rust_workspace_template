pub fn generate_pub_new_token_stream_impl(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    super::generate_modified_new_token_stream_impl::generate_modified_new_token_stream_impl(
        attr_token_stream,
        &quote::quote! {pub},
        parameters_token_stream,
        ts,
    )
}
