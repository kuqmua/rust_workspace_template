#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

#[must_use]
pub fn pg_crud_common_query_part_error_token_stream()
-> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();

    #[allow(non_snake_case, reason = "lint suppression is required here")]
    let (QueryPartErrorUpperCamelCase,) = (names.get_query_part_error_upper_camel_case(),);
    quote::quote! {pg_crud_common::query_part_error::#QueryPartErrorUpperCamelCase}.into()
}
