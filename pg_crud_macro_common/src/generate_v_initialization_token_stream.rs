#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::domain_types::*;

pub fn generate_v_initialization_token_stream(
    import: &Import,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (VSnakeCase,) = (names.get_v_snake_case(),);
    quote::quote! {#import::V { #VSnakeCase: #ts }}.into()
}
