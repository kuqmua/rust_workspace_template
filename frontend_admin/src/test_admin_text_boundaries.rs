#[test]
fn test_admin_joined_text_preserves_byte_boundaries_and_error_fallback() {
    [constants_str::X, constants_str::NON_ASCII_U_E9]
        .into_iter()
        .fold((), |(), suffix| {
            let mut input = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - suffix.len());
            input.push_str(suffix);
            let pointer = input.as_ptr();
            let result = crate::admin_joined_text::AdminJoinedText::try_from(input);
            assert!(result.is_ok());
            if let Ok(admin_text) = result {
                assert_eq!(admin_text.as_ref().len(), constants_usize::VALUE_16_777_216);
                let mut output = String::from(admin_text);
                assert_eq!(output.as_ptr(), pointer);
                assert!(output.ends_with(suffix));
                output.push_str(constants_str::X);
                assert!(matches!(crate::admin_joined_text::AdminJoinedText::try_from(output), Err(crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError::TooLong)));
            }
        });
    assert!(
        matches!(crate::admin_joined_text::AdminJoinedText::try_from(String::new()), Ok(admin_text) if admin_text.as_ref().is_empty())
    );
    let error =
        crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError::TooLong;
    assert_eq!(
        String::from(crate::admin_joined_text::AdminJoinedText::from(error)),
        error.to_string()
    );
}

#[test]
fn test_admin_ssr_text_preserves_byte_boundaries_and_error_fallback() {
    [constants_str::X, constants_str::NON_ASCII_U_E9]
        .into_iter()
        .fold((), |(), suffix| {
            let mut input = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - suffix.len());
            input.push_str(suffix);
            let pointer = input.as_ptr();
            let result = crate::admin_ssr_text::AdminSsrText::try_from(input);
            assert!(result.is_ok());
            if let Ok(admin_text) = result {
                assert_eq!(admin_text.as_ref().len(), constants_usize::VALUE_16_777_216);
                let mut output = String::from(admin_text);
                assert_eq!(output.as_ptr(), pointer);
                assert!(output.ends_with(suffix));
                output.push_str(constants_str::X);
                assert!(matches!(crate::admin_ssr_text::AdminSsrText::try_from(output), Err(crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError::TooLarge)));
            }
        });
    assert!(
        matches!(crate::admin_ssr_text::AdminSsrText::try_from(String::new()), Ok(admin_text) if admin_text.as_ref().is_empty())
    );
    let error =
        crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError::TooLarge;
    assert_eq!(
        String::from(crate::admin_ssr_text::AdminSsrText::from(error)),
        error.to_string()
    );
}
