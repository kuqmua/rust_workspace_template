#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::domain_types::*;

pub fn maybe_wrap_into_braces_token_stream(
    ts: &dyn quote::ToTokens,
    wrap: WrapIntoBraces,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    if bool::from(wrap) {
        wrap_into_scopes_token_stream(&ts)
    } else {
        quote::quote! {#ts}.into()
    }
}
