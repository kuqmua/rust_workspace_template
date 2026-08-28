#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::super::*;

pub fn generate_impl_all_variants_default_some_one_element_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (AllVariantsDefaultSomeOneElementSnakeCase,) =
        (&names.AllVariantsDefaultSomeOneElementSnakeCase,);
    let path_trait_token_stream = import.all_variants_default_some_one_element();
    let all_enum_variants = import.all_enum_variants();
    quote::quote! {
        impl #path_trait_token_stream for #identifier {
            fn #AllVariantsDefaultSomeOneElementSnakeCase() -> #all_enum_variants<Self> {
                (#ts).into()
            }
        }
    }
    .into()
}
