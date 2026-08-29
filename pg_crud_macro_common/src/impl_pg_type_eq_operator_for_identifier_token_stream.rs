#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn impl_pg_type_eq_operator_for_identifier_token_stream(
    import: &crate::import::Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_ctx::NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
    let (EqOperatorUpperCamelCase, PgTypeEqOperatorUpperCamelCase) = (
        names.get_eq_operator_upper_camel_case(),
        names.get_pg_type_eq_operator_upper_camel_case(),
    );
    quote::quote! {
        impl #import::pg_type_eq_operator::#PgTypeEqOperatorUpperCamelCase for #identifier {
            fn operator(&self) -> #import::eq_operator::#EqOperatorUpperCamelCase {
                #ts
            }
        }
    }
    .into()
}
