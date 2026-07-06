pub fn gen_impl_dflt_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl Default for #ident {
            fn default() -> Self {
                #ts
            }
        }
    }
}
