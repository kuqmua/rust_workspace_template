#[cfg(test)]
mod tests {
    #[derive(generate_constructor::New, optimal_memory_layout::OptimalMemoryLayout)]
    struct Fixture<T>
    where
        T: Copy,
    {
        first: T,
        second: Option<T>,
    }

    #[test]
    fn generates_const_new_for_named_fields_and_generics() {
        let assert_fixture = |value: Fixture<bool>| {
            assert!(value.first);
            assert_eq!(value.second, None);
        };
        assert_fixture(Fixture::new(true, None));

        let _proc_macro2_marker: Option<proc_macro2::TokenStream> = None;
        let _quote_marker = quote::quote!();
        let _syn_marker: Option<syn::DeriveInput> = None;
    }

    const _: Fixture<bool> = Fixture::new(true, None);
    const _: usize = constants_str::DOT.len();
}
