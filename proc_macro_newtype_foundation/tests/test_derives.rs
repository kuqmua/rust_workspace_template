#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        proc_macro_newtype_foundation::FromInner,
        proc_macro_newtype_foundation::GetInner,
    )]
    #[accessor(pub(crate))]
    #[borrow]
    struct BorrowedValue(bool);

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        proc_macro_newtype_foundation::FromInner,
        proc_macro_newtype_foundation::GetInner,
    )]
    #[accessor(pub(crate))]
    struct OwnedValue(usize);

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_foundation::AsRefInner,
        proc_macro_newtype_foundation::FromInner,
        proc_macro_newtype_foundation::ToTokens,
    )]
    struct TokenValue(syn::Expr);

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_foundation::AsRefInner,
        proc_macro_newtype_foundation::FromInner,
    )]
    struct ReferencedValue<'value_lt>(&'value_lt syn::Expr);

    const _: usize = constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR.len();

    #[test]
    fn test_derives_generate_from_and_borrowed_getter() {
        let tokens: proc_macro2::TokenStream = quote::quote! { true };
        let _expression =
            syn::parse2::<syn::Expr>(tokens).expect(constants_str::DIAGNOSTIC_690CF228);
        let value = BorrowedValue::from(true);
        assert!(value.get());
    }

    #[test]
    fn test_derives_generate_from_and_owned_getter() {
        let value = OwnedValue::from(1usize);
        assert_eq!(value.get(), 1usize);
    }

    #[test]
    fn test_derives_generate_as_ref_and_to_tokens() {
        let value = TokenValue::from(
            syn::parse_str::<syn::Expr>(constants_str::TRUE)
                .expect(constants_str::DIAGNOSTIC_CB6DB081),
        );
        let _: &syn::Expr = value.as_ref();

        assert_eq!(quote::quote! { #value }.to_string(), constants_str::TRUE);
    }

    #[test]
    fn test_as_ref_inner_dereferences_reference_fields() {
        let expression = syn::parse_str::<syn::Expr>(constants_str::TRUE)
            .expect(constants_str::DIAGNOSTIC_E7306EF4);
        let value = ReferencedValue::from(&expression);

        assert!(matches!(value.as_ref(), syn::Expr::Lit(_)));
    }
}
