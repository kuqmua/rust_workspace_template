#[test]
fn test_accessor_generators_reject_malformed_syntax_with_owner_diagnostics() {
    let provider_result =
        std::panic::catch_unwind(|| crate::generate_accessor_trait(quote::quote! { struct }));
    assert!(provider_result.is_err_and(|error| {
        error
            .downcast_ref::<String>()
            .is_some_and(|message| message.starts_with(constants_str::DIAGNOSTIC_195B48F5))
    }));
    let fields_result = std::panic::catch_unwind(|| {
        crate::generate_accessor_traits_for_struct_fields(quote::quote! { struct })
    });
    assert!(fields_result.is_err_and(|error| {
        error
            .downcast_ref::<String>()
            .is_some_and(|message| message.starts_with(constants_str::DIAGNOSTIC_49780295))
    }));
}

#[test]
fn test_field_provider_case_overflow_rejects_all_implementations() {
    let identifier = syn::Ident::new(
        constants_str::HELLOWORLD_ALT.repeat(104_858usize).as_str(),
        proc_macro2::Span::call_site(),
    );
    let generated = crate::generate_accessor_traits_for_struct_fields(quote::quote! {
        struct FieldCaseOverflowFixture {
            value: FixtureValue,
            #identifier: FixtureValue,
        }
    });
    assert!(
        generated
            .to_string()
            .contains(constants_str::VALUE_2EDAC0BF)
    );
    assert!(syn::parse2::<syn::File>(generated).is_ok_and(
        |file| matches!(file.items.as_slice(), [syn::Item::Macro(item)]
            if item.mac.path.segments.last().is_some_and(|segment|
                segment.ident == stringify!(compile_error)))
    ));
}
