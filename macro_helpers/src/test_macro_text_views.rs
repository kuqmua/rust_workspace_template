#[test]
fn test_string_path_adapter_preserves_canonical_segments_without_trailing_separator() {
    let segments = crate::string_syn_punct::string_syn_punct();
    let path_result = syn::parse2::<syn::Path>(quote::quote!(#segments));
    assert!(path_result.is_ok());
    assert_eq!(
        path_result.ok(),
        Some(syn::parse_quote!(std::string::String))
    );
}

#[test]
fn test_compile_error_message_string_conversion_borrows_exact_text() {
    [
        String::from(constants_str::EMPTY),
        String::from(constants_str::CURSOR_TEST_PAYLOAD),
        ['\u{e9}', '\0', '"', '\n'].into_iter().collect(),
    ]
    .into_iter()
    .fold((), |(), value| {
        let message = crate::compile_error_message::CompileErrorMessage::from(&value);
        let borrowed: &str = message.as_ref();
        assert_eq!(borrowed, value.as_str());
        assert!(std::ptr::eq(borrowed.as_ptr(), value.as_ptr()));
    });
}

#[test]
fn test_location_field_factory_preserves_private_named_field_contract() {
    let field = syn::Field::from(crate::location_syn_field::location_syn_field());
    let expected: syn::Field = syn::parse_quote!(location: location_lib::location::Location);
    assert_eq!(field, expected);
}
