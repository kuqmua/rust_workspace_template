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
#[test]
fn test_admin_error_message_preserves_display_and_underlying_byte_validation() {
    assert!(
        crate::admin_ssr_error_message::AdminSsrErrorMessage::try_from(String::new())
            .is_ok_and(|admin_ssr_error_message| admin_ssr_error_message.to_string().is_empty())
    );
    assert!(
        [constants_str::X, constants_str::NON_ASCII_U_E9]
            .into_iter()
            .all(|suffix| {
                let mut input =
                    constants_str::X.repeat(constants_usize::VALUE_1_048_576 - suffix.len());
                input.push_str(suffix);
                let preserved =
                    crate::admin_ssr_error_message::AdminSsrErrorMessage::try_from(input.clone())
                        .is_ok_and(|admin_ssr_error_message| {
                            admin_ssr_error_message.to_string() == input
                        });
                input.push_str(constants_str::X);
                let expected = to_err_string::error_text::ErrorTextTryFromStringError::TooLong {
                    len: input.len(),
                    max: constants_usize::VALUE_1_048_576,
                };
                preserved
                    && crate::admin_ssr_error_message::AdminSsrErrorMessage::try_from(input)
                        .is_err_and(|error| error == expected)
            })
    );
}
#[test]
fn test_view_rendering_enforces_html_limit_after_escaping() {
    let maximum = constants_usize::VALUE_16_777_216;
    let accepted = crate::render_view::render_view(constants_str::X.repeat(maximum));
    assert_eq!(accepted.as_ref().len(), maximum);
    assert!(accepted.as_ref().bytes().all(|byte| byte == b'x'));
    let expected =
        crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge
            .to_string();
    assert_eq!(
        crate::render_view::render_view(constants_str::X.repeat(maximum + 1usize)).as_ref(),
        expected
    );
    let expanding = std::iter::repeat_n('<', constants_usize::VALUE_8_388_608).collect::<String>();
    assert_eq!(
        crate::render_view::render_view(expanding).as_ref(),
        expected
    );
}

#[test]
fn test_document_rendering_rejects_oversized_wrapped_title() {
    assert!(crate::admin_ssr_text::AdminSsrText::try_from(constants_str::X.repeat(constants_usize::VALUE_16_777_216))
        .is_ok_and(|admin_ssr_text| {
            let output = crate::render_document::render_document(&admin_ssr_text, String::new());
            output.as_ref() == crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge.to_string()
        }));
}

#[test]
fn test_join_text_preserves_empty_elements_and_single_values() {
    assert_eq!(
        crate::join_text::join_text([constants_str::X]).as_ref(),
        constants_str::X
    );
    assert_eq!(
        crate::join_text::join_text([constants_str::EMPTY]).as_ref(),
        constants_str::EMPTY
    );
    let expected = format!(
        "{}{}{}",
        constants_str::COMMA_SPACE,
        constants_str::X,
        constants_str::COMMA_SPACE
    );
    assert_eq!(
        crate::join_text::join_text([constants_str::EMPTY, constants_str::X, constants_str::EMPTY])
            .as_ref(),
        expected
    );
}

#[test]
fn test_join_text_counts_separator_bytes_at_output_limit() {
    [constants_str::X, constants_str::NON_ASCII_U_E9].into_iter().fold((), |(), suffix| {
        let mut input = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - constants_str::COMMA_SPACE.len() - suffix.len());
        input.push_str(suffix);
        let joined = crate::join_text::join_text([input.as_str(), constants_str::EMPTY]);
        assert_eq!(joined.as_ref().len(), constants_usize::VALUE_16_777_216);
        assert!(joined.as_ref().starts_with(input.as_str()));
        assert!(joined.as_ref().ends_with(constants_str::COMMA_SPACE));
        input.push_str(constants_str::X);
        assert_eq!(crate::join_text::join_text([input.as_str(), constants_str::EMPTY]).as_ref(), crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError::TooLong.to_string());
    });
}
