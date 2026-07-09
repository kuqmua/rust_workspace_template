pub fn gen_impl_to_err_string_ts(
    impl_generics_ts: &dyn quote::ToTokens,
    ident_ts: &dyn quote::ToTokens,
    ident_generics_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_ts::GeneratedRustTs {
    let self_sc = naming::SelfSc;
    let to_err_string_sc = naming::ToErrStringSc;
    let to_err_string_ucc = naming::ToErrStringUcc;
    quote::quote! {
        impl #impl_generics_ts to_err_string::#to_err_string_ucc for #ident_ts #ident_generics_ts {
            fn #to_err_string_sc(&#self_sc) -> to_err_string::ToErrStringValue {
                to_err_string::ToErrStringValue::try_from(#ts).unwrap_or_else(to_err_string::ToErrStringValue::from)
            }
        }
    }
    .into()
}
