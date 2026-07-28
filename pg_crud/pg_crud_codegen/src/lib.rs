#[must_use]
pub fn operator_tokens(
    value: pg_crud_common::Operator,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(match value {
        pg_crud_common::Operator::And => quote::quote! { And },
        pg_crud_common::Operator::AndNot => quote::quote! { AndNot },
        pg_crud_common::Operator::Or => quote::quote! { Or },
        pg_crud_common::Operator::OrNot => quote::quote! { OrNot },
    })
}

#[must_use]
pub fn greater_than_variant_tokens(
    value: pg_crud_common::PgTypeGreaterThanVariant,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(match value {
        pg_crud_common::PgTypeGreaterThanVariant::EqNotGreaterThan => {
            quote::quote! { EqNotGreaterThan }
        }
        pg_crud_common::PgTypeGreaterThanVariant::GreaterThan => quote::quote! { GreaterThan },
        pg_crud_common::PgTypeGreaterThanVariant::NotGreaterThan => {
            quote::quote! { NotGreaterThan }
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn variant_tokens_are_stable() {
        assert_eq!(
            super::operator_tokens(pg_crud_common::Operator::AndNot).to_string(),
            "AndNot"
        );
        assert_eq!(
            super::greater_than_variant_tokens(
                pg_crud_common::PgTypeGreaterThanVariant::EqNotGreaterThan,
            )
            .to_string(),
            "EqNotGreaterThan"
        );
    }
}
