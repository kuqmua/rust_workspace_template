#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_impl_default_some_one_element_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import: &crate::import::Import,
    identifier: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();

    #[allow(
        non_snake_case,
        reason = "generate impl default some one element token stream requires this localized allowance for generated or framework-constrained code verified by focused tests"
    )]
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
