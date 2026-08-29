#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_v_initialization_token_stream(
    import: &crate::import::Import,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_ctx::NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (VSnakeCase,) = (names.get_v_snake_case(),);
    quote::quote! {#import::v::V { #VSnakeCase: #ts }}.into()
}
