pub fn generate_impl_crate_is_string_empty_for_identifier_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        impl pg_crud_common::is_string_empty::IsStringEmpty for #identifier {
            fn is_string_empty(&self) -> pg_crud_common::is_string_empty_result::IsStringEmptyResult {
                pg_crud_common::is_string_empty_result::IsStringEmptyResult::from(#ts)
            }
        }
    }
    .into()
}
