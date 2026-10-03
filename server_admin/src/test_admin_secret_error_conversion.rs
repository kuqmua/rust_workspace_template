#[test]
fn test_admin_string_error_conversion_preserves_validation_category() {
    let cases = [
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::InvalidBounds,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooShort {
                len: 0usize,
                min: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooShort,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooLong {
                len: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooLong,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::ContainsNul,
            crate::admin_secret_text_error::AdminSecretTextError::ContainsNul,
        ),
        (
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidValue,
            crate::admin_secret_text_error::AdminSecretTextError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(|(source, expected)| {
        crate::admin_secret_text_error::AdminSecretTextError::from(source) == expected
    }));
}

#[test]
fn test_admin_cookie_error_conversion_preserves_validation_category() {
    let cases = [
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::InvalidBounds,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::TooShort {
                len: 0usize,
                min: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooShort,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::TooLong {
                len: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooLong,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::ContainsNul,
            crate::admin_secret_text_error::AdminSecretTextError::ContainsNul,
        ),
        (
            crate::std_admin_cookie::StdAdminCookieTryFromStringError::InvalidValue,
            crate::admin_secret_text_error::AdminSecretTextError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(|(source, expected)| {
        crate::admin_secret_text_error::AdminSecretTextError::from(source) == expected
    }));
}

#[test]
fn test_admin_access_token_error_conversion_preserves_validation_category() {
    let cases = [
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::InvalidBounds,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooShort {
                len: 0usize,
                min: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooShort,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooLong {
                len: 2usize,
                max: 1usize,
            },
            crate::admin_secret_text_error::AdminSecretTextError::TooLong,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::ContainsNul,
            crate::admin_secret_text_error::AdminSecretTextError::ContainsNul,
        ),
        (
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidValue,
            crate::admin_secret_text_error::AdminSecretTextError::InvalidValue,
        ),
    ];
    assert!(cases.into_iter().all(|(source, expected)| {
        crate::admin_secret_text_error::AdminSecretTextError::from(source) == expected
    }));
}
