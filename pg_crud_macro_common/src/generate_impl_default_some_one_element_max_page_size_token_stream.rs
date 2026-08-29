#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::domain_types::*;

pub fn generate_impl_default_some_one_element_max_page_size_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import: &Import,
    identifier: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (DefaultSomeOneElementMaxPageSizeSnakeCase,) =
        (names.get_default_some_one_element_max_page_size_snake_case(),);
    let path_trait_token_stream = import.default_some_one_element_max_page_size();
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #identifier #identifier_generic_token_stream {
            fn #DefaultSomeOneElementMaxPageSizeSnakeCase() -> Self {
                #ts
            }
        }
    }
    .into()
}
