#[test]
fn test_provider_declarations_preserve_generics_and_bounds() {
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct ForwardCollisionFixture<'value, ForwardCollisionFixture0, const COUNT: usize>(
            &'value ForwardCollisionFixture0
        ) where ForwardCollisionFixture0: Copy;
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if matches!(file.items.as_slice(), [syn::Item::Trait(provider), syn::Item::Impl(forwarding)]
            if provider.generics.params.len() == 3usize
                && provider.generics.where_clause.is_some()
                && forwarding.generics.params.len() == 4usize
                && forwarding.generics.where_clause.is_some())));
}

#[test]
fn test_field_provider_implementations_preserve_owner_generics() {
    let generated = crate::generate_accessor_traits_for_struct_fields(quote::quote! {
        struct ConstStateFixture<const COUNT: usize> {
            value: FixtureValue,
        }
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if file.items.len() == 2usize && file.items.iter().all(|item|
            matches!(item, syn::Item::Impl(implementation)
                if implementation.generics.params.len() == 1usize))));
}
#[test]
fn test_field_providers_accept_raw_identifiers() {
    let generated = crate::generate_accessor_traits_for_struct_fields(quote::quote! {
        struct RawFieldFixture {
            r#type: FixtureValue,
        }
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if file.items.len() == 2usize));
}
#[test]
fn test_provider_forwarding_does_not_shadow_field_type() {
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct ForwardFieldFixture(ForwardFieldFixture0);
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if matches!(file.items.as_slice(), [syn::Item::Trait(_), syn::Item::Impl(forwarding)]
            if matches!(forwarding.generics.params.first(), Some(syn::GenericParam::Type(parameter))
                if parameter.ident != quote::format_ident!("ForwardFieldFixture0")))));
}

#[test]
fn test_oversized_case_conversion_emits_compile_error() {
    let identifier = syn::Ident::new(
        constants_str::HELLOWORLD_ALT.repeat(104_858usize).as_str(),
        proc_macro2::Span::call_site(),
    );
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct #identifier(u8);
    });
    assert!(
        generated
            .to_string()
            .contains(constants_str::VALUE_2EDAC0BF)
    );
}
