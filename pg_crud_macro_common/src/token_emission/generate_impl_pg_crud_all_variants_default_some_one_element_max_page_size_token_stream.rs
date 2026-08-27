#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::super::*;

pub fn generate_impl_pg_crud_all_variants_default_some_one_element_max_page_size_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_all_variants_default_some_one_element_max_page_size_token_stream(
        &Import::PgCrudCommon,
        identifier,
        ts,
    )
}
