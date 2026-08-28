#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::super::*;

#[must_use]
pub fn pg_crud_common_query_part_error_token_stream()
-> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (QueryPartErrorUpperCamelCase,) = (&names.QueryPartErrorUpperCamelCase,);
    quote::quote! {pg_crud_common::domain_types::#QueryPartErrorUpperCamelCase}.into()
}
