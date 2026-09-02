#[test]
fn test_bounded_string_missing_max_returns_compile_error() {
    let input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(min = 1)]
        struct Value(String);
    };
    let result = crate::generate_bounded_string_token_stream(
        crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&input),
    );
    assert!(result.is_err(), "29f8ddc2");
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            constants_str::MACRO_DIAGNOSTICS_BOUNDED_STRING_MAX_ERROR
        );
    }
}
#[test]
fn test_bounded_string_utoipa_byte_length_returns_compile_error() {
    let input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(max = 4, utoipa)]
        struct Value(bounded_types::bounded_string::BoundedString<0usize, 4usize, false>);
    };
    let result = crate::generate_bounded_string_token_stream(
        crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&input),
    );
    assert!(result.is_err(), "da6f2151");
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            constants_str::BOUNDEDSTRING_UTOIPA_REQUIRES_CHARS_SO_OPENAPI_LENGTH_SEMANTICS_MATCH_RUNTIME
        );
    }
}
#[test]
fn test_duplicate_options_preserve_attribute_diagnostic() {
    let bounded_input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(max = 4, trim, trim)]
        struct BoundedValue(String);
    };
    let bounded_result = crate::generate_bounded_string_token_stream(
        crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&bounded_input),
    );
    if let Err(error) = bounded_result {
        assert_eq!(
            error.to_string(),
            constants_str::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR
        );
    } else {
        std::panic::panic_any(constants_str::PANIC_D03CED5C);
    }
}
