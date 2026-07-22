pub fn generate_impl_default_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        impl Default for #identifier {
            fn default() -> Self {
                #ts
            }
        }
    }
    .into()
}
