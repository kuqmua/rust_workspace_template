#[cfg(test)]
mod tests {
    #[test]
    fn required_bounded_text_rejects_nul() {
        assert_eq!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                constants_str::test_fixtures::TEST_TEXT_WITH_NUL.to_owned()
            ),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::ContainsNul)
        );
    }

    #[test]
    fn fixed_hex_requires_lowercase_and_exact_length() {
        let _value = crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(
            constants_str::test_fixtures::TEST_GIT_COMMIT_HASH.to_owned(),
        )
        .expect("fdb4f77c fixed_hex_requires_lowercase_and_exact_length invariant must hold");
        assert_eq!(
            crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(
                constants_str::test_fixtures::TEST_UPPERCASE_GIT_COMMIT_HASH.to_owned()
            ),
            Err(crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidSymbol)
        );
    }

    #[test]
    fn url_safe_token_policy_is_table_driven() {
        assert_eq!(
            [
                constants_str::catalog::ABC_ALT_3,
                constants_str::test_fixtures::TEST_URL_TOKEN_WITH_SEPARATOR,
                "",
            ]
            .map(|value| {
                crate::validate_url_safe_token_part::validate_url_safe_token_part(
                    crate::url_safe_token_part_ref::UrlSafeTokenPartRef::from(value),
                    crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                        128usize,
                    ),
                )
            }),
            [
                Ok(()),
                Err(
                    crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::InvalidSymbol
                ),
                Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::Empty),
            ]
        );
        assert_eq!(
            crate::validate_url_safe_token_part::validate_url_safe_token_part(
                crate::url_safe_token_part_ref::UrlSafeTokenPartRef::from(
                    constants_str::catalog::ABC_ALT_3
                ),
                crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                    2usize
                ),
            ),
            Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong)
        );
    }

    #[test]
    fn password_policy_is_table_driven() {
        let range = crate::password_length_range::PasswordLengthRange::from_prevalidated(
            crate::password_length::PasswordLength::from(12usize),
            crate::password_length::PasswordLength::from(128usize),
        );
        assert_eq!(
            [
                constants_str::test_fixtures::TEST_STRONG_PASSWORD,
                constants_str::catalog::PASSWORD,
                constants_str::test_fixtures::TEST_PASSWORD_WITH_WHITESPACE,
            ]
            .map(|value| {
                crate::validate_password_policy::validate_password_policy(
                    crate::password_text_ref::PasswordTextRef::from(value),
                    range,
                )
            }),
            [
                Ok(()),
                Err(crate::password_policy_violation::PasswordPolicyViolation::TooShort),
                Err(crate::password_policy_violation::PasswordPolicyViolation::ContainsWhitespace),
            ]
        );
        let secret = constants_str::catalog::NEVER_PRINT_THIS_VALUE;
        assert!(
            !format!(
                "{:?}",
                crate::password_text_ref::PasswordTextRef::from(secret)
            )
            .contains(secret)
        );
    }
}
