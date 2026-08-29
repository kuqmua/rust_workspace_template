#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn maybe_wrap_into_braces_token_stream(
    ts: &dyn quote::ToTokens,
    wrap: crate::wrap_into_braces::WrapIntoBraces,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    if bool::from(wrap) {
        crate::wrap_into_scopes_token_stream::wrap_into_scopes_token_stream(&ts)
    } else {
        quote::quote! {#ts}.into()
    }
}
