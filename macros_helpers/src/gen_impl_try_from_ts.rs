pub fn gen_impl_try_from_ts(
    from_type_ts: &dyn quote::ToTokens,
    for_type_ts: &dyn quote::ToTokens,
    er_type_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::GeneratedRustTs {
    let v_sc = naming::VSc;
    quote::quote! {
        impl TryFrom<#from_type_ts> for #for_type_ts {
            type Error = #er_type_ts;
            fn try_from(#v_sc: #from_type_ts) -> Result<Self, Self::Error> {
                #ts
            }
        }
    }
    .into()
}
