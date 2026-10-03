#[test]
fn test_authorization_collection_error_conversion_preserves_coarse_category() {
    let cases = [
        bounded_types::bounded_value_error::BoundedValueError::AboveMax {
            actual: bounded_types::bounded_len::BoundedLen::from(2usize),
            max: bounded_types::bounded_len::BoundedLen::from(1usize),
        },
        bounded_types::bounded_value_error::BoundedValueError::BelowMin {
            actual: bounded_types::bounded_len::BoundedLen::from(0usize),
            min: bounded_types::bounded_len::BoundedLen::from(1usize),
        },
        bounded_types::bounded_value_error::BoundedValueError::InvalidBounds {
            min: bounded_types::bounded_len::BoundedLen::from(2usize),
            max: bounded_types::bounded_len::BoundedLen::from(1usize),
        },
    ];
    cases.into_iter().for_each(|bounded_value_error| {
        let error = crate::admin_auth_collection_error::AdminAuthCollectionError::from(bounded_value_error);
        assert_eq!(error, crate::admin_auth_collection_error::AdminAuthCollectionError::TooLarge);
        assert_eq!(error.to_string(), constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHORIZATION_COLLECTION_EXCEEDS_MAXIMUM_LENGTH);
        assert!(std::error::Error::source(&error).is_none());
    });
}

#[test]
fn test_authorization_positive_value_error_conversion_preserves_zero_category() {
    let error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::from(
        server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid,
    );
    assert_eq!(
        error,
        crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero
    );
    assert_eq!(error.to_string(), stringify!(Zero));
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn test_html_form_key_and_text_errors_preserve_coarse_length_categories() {
    let cases = [
        bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
            actual_length: bounded_types::bounded_len::BoundedLen::from(2usize),
            maximum_length: bounded_types::bounded_len::BoundedLen::from(1usize),
        },
        bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
            actual_length: bounded_types::bounded_len::BoundedLen::from(0usize),
            minimum_length: bounded_types::bounded_len::BoundedLen::from(1usize),
        },
    ];
    cases.into_iter().for_each(|bounded_string_error| {
        let key_error =
            crate::admin_html_form_key_error::AdminHtmlFormKeyError::from(bounded_string_error);
        assert!(matches!(
            key_error,
            crate::admin_html_form_key_error::AdminHtmlFormKeyError::TooLong
        ));
        assert_eq!(
            key_error.to_string(),
            constants_str::ADMIN_HTML_FORM_KEY_TOO_LONG
        );
        assert!(std::error::Error::source(&key_error).is_none());
        let text_error =
            crate::admin_html_form_text_error::AdminHtmlFormTextError::from(bounded_string_error);
        assert!(matches!(
            text_error,
            crate::admin_html_form_text_error::AdminHtmlFormTextError::TooLong
        ));
        assert_eq!(
            text_error.to_string(),
            constants_str::ADMIN_HTML_FORM_TEXT_TOO_LONG
        );
        assert!(std::error::Error::source(&text_error).is_none());
    });
}
