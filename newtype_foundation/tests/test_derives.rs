#[cfg(test)]
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        newtype_foundation::FromInner,
        newtype_foundation::GetInner,
    )]
    #[accessor(pub(crate))]
    #[borrow]
    struct BorrowedValue(bool);

    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        newtype_foundation::FromInner,
        newtype_foundation::GetInner,
    )]
    #[accessor(pub(crate))]
    struct OwnedValue(usize);

    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        newtype_foundation::AsRefInner,
        newtype_foundation::FromInner,
        newtype_foundation::ToTokens,
    )]
    struct TokenValue(syn::Expr);

    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        newtype_foundation::AsRefInner,
        newtype_foundation::FromInner,
    )]
    struct ReferencedValue<'value_lt>(&'value_lt syn::Expr);

    const _: usize = constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR.len();

    #[test]
    fn test_derives_generate_from_and_borrowed_getter() {
        let tokens: proc_macro2::TokenStream = quote::quote! { true };
        let _expression = syn::parse2::<syn::Expr>(tokens)
            .expect("690cf228 foundation derive dependency invariant must hold");
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
                .expect("cb6db081 bool must parse as a Rust expression"),
        );
        let _: &syn::Expr = value.as_ref();

        assert_eq!(quote::quote! { #value }.to_string(), constants_str::TRUE);
    }

    #[test]
    fn test_as_ref_inner_dereferences_reference_fields() {
        let expression = syn::parse_str::<syn::Expr>(constants_str::TRUE)
            .expect("e7306ef4 bool must parse as a Rust expression");
        let value = ReferencedValue::from(&expression);

        assert!(matches!(value.as_ref(), syn::Expr::Lit(_)));
    }
}
