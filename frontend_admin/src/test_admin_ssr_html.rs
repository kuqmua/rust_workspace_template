#[test]
fn test_admin_ssr_html_accepts_exact_byte_limit_without_copying() {
    let input = constants_str::X.repeat(constants_usize::VALUE_16_777_216);
    let pointer = input.as_ptr();
    let result = crate::admin_ssr_html::AdminSsrHtml::try_from(input);
    assert!(result.is_ok());
    if let Ok(admin_ssr_html) = result {
        assert_eq!(
            admin_ssr_html.as_ref().len(),
            constants_usize::VALUE_16_777_216
        );
        let output = String::from(admin_ssr_html);
        assert_eq!(output.as_ptr(), pointer);
    }
}

#[test]
fn test_admin_ssr_html_rejects_one_byte_over_limit() {
    assert!(matches!(
        crate::admin_ssr_html::AdminSsrHtml::try_from(
            constants_str::X.repeat(constants_usize::VALUE_16_777_216 + constants_usize::ONE)
        ),
        Err(crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge)
    ));
}

#[test]
fn test_admin_ssr_html_counts_unicode_bytes_and_preserves_fallback() {
    let mut input = constants_str::X
        .repeat(constants_usize::VALUE_16_777_216 - constants_str::NON_ASCII_U_E9.len());
    input.push_str(constants_str::NON_ASCII_U_E9);
    let result = crate::admin_ssr_html::AdminSsrHtml::try_from(input);
    assert!(result.is_ok());
    if let Ok(admin_ssr_html) = result {
        let mut output = String::from(admin_ssr_html);
        assert!(output.ends_with(constants_str::NON_ASCII_U_E9));
        output.push_str(constants_str::X);
        assert!(matches!(crate::admin_ssr_html::AdminSsrHtml::try_from(output), Err(crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge)));
    }
    let error =
        crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge;
    let expected = error.to_string();
    assert_eq!(
        String::from(crate::admin_ssr_html::AdminSsrHtml::from(error)),
        expected
    );
    assert!(
        matches!(crate::admin_ssr_html::AdminSsrHtml::try_from(String::new()), Ok(admin_ssr_html) if admin_ssr_html.as_ref().is_empty())
    );
}
