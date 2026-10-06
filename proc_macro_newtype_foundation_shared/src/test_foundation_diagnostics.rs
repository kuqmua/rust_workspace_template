#[test]
fn test_foundation_derives_reject_every_non_single_tuple_shape() {
    let message = constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR;
    let expected = quote::quote! { ::core::compile_error! { #message } }.to_string();
    [
        quote::quote! { struct FoundationNamedDiagnosticFixture { value: bool } },
        quote::quote! { struct FoundationUnitDiagnosticFixture; },
        quote::quote! { enum FoundationEnumDiagnosticFixture { Value } },
        quote::quote! { struct FoundationEmptyTupleDiagnosticFixture(); },
        quote::quote! { struct FoundationMultipleTupleDiagnosticFixture(bool, bool); },
    ]
    .into_iter()
    .fold((), |(), input| {
        assert!(
            [
                crate::foundation_as_ref_inner(input.clone()),
                crate::foundation_from_inner(input.clone()),
                crate::foundation_get_inner(input.clone()),
                crate::foundation_to_tokens(input),
            ]
            .into_iter()
            .all(|expanded| expanded.to_string() == expected)
        );
    });
}

#[test]
fn test_foundation_derives_preserve_parse_failure_diagnostics() {
    let input = quote::quote! { struct };
    let expected = syn::parse2::<syn::DeriveInput>(input.clone())
        .err()
        .map(syn::Error::into_compile_error)
        .map(|tokens| tokens.to_string());
    assert!(expected.is_some());
    assert!(
        [
            crate::foundation_as_ref_inner(input.clone()),
            crate::foundation_from_inner(input.clone()),
            crate::foundation_get_inner(input.clone()),
            crate::foundation_to_tokens(input),
        ]
        .into_iter()
        .all(|expanded| Some(expanded.to_string()) == expected)
    );
}

#[test]
fn test_foundation_getter_preserves_invalid_accessor_visibility_diagnostic() {
    let invalid_visibility = quote::quote! { pub(in) };
    let expected = syn::parse2::<syn::Visibility>(invalid_visibility.clone())
        .err()
        .map(syn::Error::into_compile_error)
        .map(|tokens| tokens.to_string());
    assert!(expected.is_some());
    let expanded = crate::foundation_get_inner(quote::quote! {
        #[accessor(#invalid_visibility)]
        struct FoundationVisibilityDiagnosticFixture(bool);
    });
    assert_eq!(Some(expanded.to_string()), expected);
}
