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
}
