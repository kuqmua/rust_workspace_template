#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_explicit_value_initialization_token_stream(
    import: &crate::import::Import,
    token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {#import::explicit_value::ExplicitValue::new(#token_stream)}.into()
}
