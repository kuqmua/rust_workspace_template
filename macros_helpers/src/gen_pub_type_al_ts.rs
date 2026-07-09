pub fn gen_pub_type_al_ts(
    al_type_name_ts: &dyn quote::ToTokens,
    al_actual_type_name_ts: &dyn quote::ToTokens,
) -> crate::generated_rust_ts::GeneratedRustTs {
    quote::quote! {pub type #al_type_name_ts = #al_actual_type_name_ts;}.into()
}
