pub fn gen_impl_dflt_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_ts::GeneratedRustTs {
    quote::quote! {
        impl Default for #ident {
            fn default() -> Self {
                #ts
            }
        }
    }
    .into()
}
