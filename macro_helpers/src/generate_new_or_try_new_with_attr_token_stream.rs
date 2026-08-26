pub(super) fn with_attr_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    quote::quote! {
        #attr_token_stream
        #ts
    }
    .into()
}
