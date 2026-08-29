#[test]
fn bounded_string_missing_max_returns_compile_error() {
    let input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(min = 1)]
        struct Value(String);
    };
    let result = crate::generate_bounded_string_token_stream(
        crate::syn_derive_input_ref::SynDeriveInputRef::from(&input),
    );
    assert!(result.is_err(), "29f8ddc2");
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            "BoundedString requires #[bounded_string(max = ...)]"
        );
    }
}
#[test]
fn bounded_string_utoipa_byte_length_returns_compile_error() {
    let input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(max = 4, utoipa)]
        struct Value(String);
    };
    let result = crate::generate_bounded_string_token_stream(
        crate::syn_derive_input_ref::SynDeriveInputRef::from(&input),
    );
    assert!(result.is_err(), "da6f2151");
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            "BoundedString utoipa requires chars so OpenAPI length semantics match runtime"
        );
    }
}
#[test]
fn duplicate_options_preserve_attribute_diagnostic() {
    let bounded_input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(max = 4, trim, trim)]
        struct BoundedValue(String);
    };
    let bounded_result = crate::generate_bounded_string_token_stream(
        crate::syn_derive_input_ref::SynDeriveInputRef::from(&bounded_input),
    );
    if let Err(error) = bounded_result {
        assert_eq!(error.to_string(), "duplicate bounded_string option");
    } else {
        panic!("d03ced5c");
    }
}
