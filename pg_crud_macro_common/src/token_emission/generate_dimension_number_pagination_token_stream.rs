#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::super::*;

#[must_use]
pub fn generate_dimension_number_pagination_token_stream(
    dimension_number: DimensionNumber,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let identifier = quote::format_ident!("dimension{}_pagination", dimension_number.get());
    quote::quote! {#identifier}.into()
}
