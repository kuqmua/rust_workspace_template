#[cfg(test)]
mod tests {
    #[test]
    fn test_required_bounded_text_rejects_nul() {
        assert_eq!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                constants_str::TEST_TEXT_WITH_NUL.to_owned()
            ),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::ContainsNul)
        );
    }

    #[test]
    fn test_fixed_hex_requires_lowercase_and_exact_length() {
        let _value = crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(
            constants_str::TEST_GIT_COMMIT_HASH.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_FDB4F77C);
        assert_eq!(
            crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(
                constants_str::TEST_UPPERCASE_GIT_COMMIT_HASH.to_owned()
            ),
            Err(crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidSymbol)
        );
    }

    #[test]
    fn test_url_safe_token_policy_is_table_driven() {
        assert_eq!(
            [
                constants_str::ABC_ALT_3,
                constants_str::TEST_URL_TOKEN_WITH_SEPARATOR,
                constants_str::EMPTY,
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
                crate::url_safe_token_part_ref::UrlSafeTokenPartRef::from(constants_str::ABC_ALT_3),
                crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                    2usize
                ),
            ),
            Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong)
        );
    }

    #[test]
    fn test_password_policy_is_table_driven() {
        let range = crate::password_length_range::PasswordLengthRange::from_prevalidated(
            crate::password_length::PasswordLength::from(12usize),
            crate::password_length::PasswordLength::from(128usize),
        );
        assert_eq!(
            [
                constants_str::TEST_STRONG_PASSWORD,
                constants_str::PASSWORD,
                constants_str::TEST_PASSWORD_WITH_WHITESPACE,
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
        let secret = constants_str::NEVER_PRINT_THIS_VALUE;
        assert!(
            !format!(
                "{:?}",
                crate::password_text_ref::PasswordTextRef::from(secret)
            )
            .contains(secret)
        );
    }

    #[test]
    fn test_password_policy_rejects_unicode_whitespace() {
        let range = crate::password_length_range::PasswordLengthRange::from_prevalidated(
            crate::password_length::PasswordLength::from(12usize),
            crate::password_length::PasswordLength::from(128usize),
        );
        let mut password = constants_str::TEST_STRONG_PASSWORD.to_owned();
        password.extend(char::from_u32(0x00A0u32));
        assert_eq!(
            crate::validate_password_policy::validate_password_policy(
                crate::password_text_ref::PasswordTextRef::from(password.as_str()),
                range,
            ),
            Err(crate::password_policy_violation::PasswordPolicyViolation::ContainsWhitespace),
        );
    }

    #[test]
    fn test_password_policy_counts_characters_for_length_bounds() {
        let range = crate::password_length_range::PasswordLengthRange::from_prevalidated(
            crate::password_length::PasswordLength::from(12usize),
            crate::password_length::PasswordLength::from(1024usize),
        );
        let short_password = constants_str::TEST_STRONG_PASSWORD
            .chars()
            .enumerate()
            .filter_map(|(index, character)| {
                (index != 1usize && index != 2usize).then_some(character)
            })
            .chain(char::from_u32(0x430u32))
            .collect::<String>();
        assert_eq!(short_password.chars().count(), 11usize);
        assert_eq!(
            crate::validate_password_policy::validate_password_policy(
                crate::password_text_ref::PasswordTextRef::from(short_password.as_str()),
                range,
            ),
            Err(crate::password_policy_violation::PasswordPolicyViolation::TooShort),
        );
        let mut maximum_password = constants_str::TEST_STRONG_PASSWORD.to_owned();
        maximum_password.extend(char::from_u32(0x430u32).into_iter().cycle().take(1012usize));
        assert_eq!(maximum_password.chars().count(), 1024usize);
        assert_eq!(
            crate::validate_password_policy::validate_password_policy(
                crate::password_text_ref::PasswordTextRef::from(maximum_password.as_str()),
                range,
            ),
            Ok(()),
        );
    }
}
