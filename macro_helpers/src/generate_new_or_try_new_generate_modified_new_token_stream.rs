pub(super) fn generate_modified_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    modifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let new_token_stream = super::generate_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        ts,
    );
    super::with_attr_token_stream_impl::with_attr_token_stream(
        attr_token_stream,
        &quote::quote! {#modifier_token_stream #new_token_stream},
    )
}
