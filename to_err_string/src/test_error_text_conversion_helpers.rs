#[test]
fn test_static_and_borrowed_error_text_preserve_content_and_byte_bounds() {
    [
        constants_str::EMPTY,
        constants_str::NON_ASCII_U_E9,
        constants_str::TEST_TEXT_WITH_NUL,
    ]
    .into_iter()
    .fold((), |(), text| {
        assert_eq!(
            crate::static_str_to_owned::static_str_to_owned(
                crate::static_str_to_owned_input::StaticStrToOwnedInput::from(text)
            )
            .as_ref(),
            text
        );
        assert_eq!(
            crate::as_ref_str_to_owned::as_ref_str_to_owned(text).as_ref(),
            text
        );
    });
    let maximum = crate::error_text_max_len::ERROR_TEXT_MAX_LEN;
    let mut input = constants_str::X.repeat(maximum - '\u{00e9}'.len_utf8());
    input.push('\u{00e9}');
    assert_eq!(input.len(), maximum);
    assert_eq!(
        crate::as_ref_str_to_owned::as_ref_str_to_owned(&input).as_ref(),
        input
    );
    input.push('\u{00e9}');
    let expected = crate::error_text::ErrorTextTryFromStringError::TooLong {
        len: input.len(),
        max: maximum,
    }
    .to_string();
    assert_eq!(
        crate::as_ref_str_to_owned::as_ref_str_to_owned(&input).as_ref(),
        expected
    );
}

#[test]
fn test_debug_error_text_limits_include_formatting_expansion() {
    let maximum = crate::error_text_max_len::ERROR_TEXT_MAX_LEN;
    let input = constants_str::X.repeat(maximum - 2usize);
    let output = crate::debug_to_string::debug_to_string(&input);
    assert_eq!(output.as_ref().len(), maximum);
    assert!(output.as_ref().starts_with('"') && output.as_ref().ends_with('"'));
    assert!(
        output
            .as_ref()
            .chars()
            .skip(1usize)
            .take(maximum - 2usize)
            .all(|character| character == 'x')
    );
    let oversized = constants_str::X.repeat(maximum - 1usize);
    let expected = crate::error_text::ErrorTextTryFromStringError::TooLong {
        len: maximum + 1usize,
        max: maximum,
    }
    .to_string();
    assert_eq!(
        crate::debug_to_string::debug_to_string(&oversized).as_ref(),
        expected
    );
}
