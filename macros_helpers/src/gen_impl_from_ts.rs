pub fn gen_impl_from_ts(
    from_type_ts: &dyn quote::ToTokens,
    for_type_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_ts::GeneratedRustTs {
    let v_sc = naming::VSc;
    quote::quote! {
        impl From<#from_type_ts> for #for_type_ts {
            fn from(#v_sc: #from_type_ts) -> Self {
                #ts
            }
        }
    }
    .into()
}
