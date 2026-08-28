#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use crate::domain_types::*;

pub fn generate_impl_default_some_one_element_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import: &Import,
    identifier: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (DefaultSomeOneElementSnakeCase,) = (names.get_default_some_one_element_snake_case(),);
    let path_trait_token_stream = import.default_some_one_element();
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #identifier #identifier_generic_token_stream {
            fn #DefaultSomeOneElementSnakeCase() -> Self {
                #ts
            }
        }
    }
    .into()
}
