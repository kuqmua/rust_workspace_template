#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_impl_all_variants_default_some_one_element_max_page_size_token_stream(
    import: &crate::import::Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();

    #[allow(
        non_snake_case,
        reason = "generate impl all variants default some one element max page size token stream requires this localized allowance for generated or framework-constrained code verified by focused tests"
    )]
    let (AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,) =
        (names.get_all_variants_default_some_one_element_max_page_size_snake_case(),);
    let path_trait_token_stream = import.all_variants_default_some_one_element_max_page_size();
    let all_enum_variants = import.all_enum_variants();
    let all_variants_default_some_one_element_max_page_size_snake_case =
        AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #identifier {
            fn #all_variants_default_some_one_element_max_page_size_snake_case() -> #all_enum_variants<Self> {
                (#ts).into()
            }
        }
    }
    .into()
}
