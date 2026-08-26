pub fn generate_try_new_token_stream_impl(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    quote::quote! {
        #attr_token_stream
        fn try_new(#parameters_token_stream) -> Result<Self, #err_type_token_stream> {
            #ts
        }
    }
    .into()
}
