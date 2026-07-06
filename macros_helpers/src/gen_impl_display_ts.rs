pub fn gen_impl_display_ts(
    impl_generics_ts: &dyn quote::ToTokens,
    ident_ts: &dyn quote::ToTokens,
    ident_generics_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let self_sc = naming::SelfSc;
    quote::quote! {
        impl #impl_generics_ts std::fmt::Display for #ident_ts #ident_generics_ts {
            fn fmt(&#self_sc, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #ts
            }
        }
    }
}
