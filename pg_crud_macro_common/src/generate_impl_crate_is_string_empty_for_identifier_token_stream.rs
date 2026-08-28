pub fn generate_impl_crate_is_string_empty_for_identifier_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        impl pg_crud_common::domain_types::IsStringEmpty for #identifier {
            fn is_string_empty(&self) -> pg_crud_common::domain_types::IsStringEmptyRes {
                pg_crud_common::domain_types::IsStringEmptyRes::from(#ts)
            }
        }
    }
    .into()
}
