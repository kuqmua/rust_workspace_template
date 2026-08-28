#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use crate::domain_types::*;

pub fn impl_pg_type_eq_operator_for_identifier_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (EqOperatorUpperCamelCase, PgTypeEqOperatorUpperCamelCase) = (
        names.get_eq_operator_upper_camel_case(),
        names.get_pg_type_eq_operator_upper_camel_case(),
    );
    quote::quote! {
        impl #import::#PgTypeEqOperatorUpperCamelCase for #identifier {
            fn operator(&self) -> #import::#EqOperatorUpperCamelCase {
                #ts
            }
        }
    }
    .into()
}
