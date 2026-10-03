#[cfg(test)]
mod tests {
    #[test]
    fn test_https_url_validator_checks_authority_and_port() {
        let mut valid_port = constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_owned();
        valid_port.push(':');
        valid_port.push_str(u16::MAX.to_string().as_str());
        let mut invalid_port = constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_owned();
        invalid_port.push(':');
        invalid_port.push_str((u32::from(u16::MAX) + 1).to_string().as_str());
        let mut whitespace = constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_owned();
        whitespace.push(' ');
        let values = [
            (constants_str::HTTPS_ADMIN_EXAMPLE_COM, Ok(())),
            (constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH, Ok(())),
            (
                constants_str::HTTPS_ADMIN_EXAMPLE_COM_WITH_INVALID_PORT,
                Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
            ),
            (
                constants_str::HTTPS_ADMIN_EXAMPLE_COM_WITH_USERINFO,
                Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
            ),
        ];
        assert!(values.into_iter().all(|(value, expected)| {
            crate::validate_https_url_text::validate_https_url_text(
                crate::https_url_text_ref::HttpsUrlTextRef::from(value),
            ) == expected
        }));
        assert_eq!(
            crate::validate_https_url_text::validate_https_url_text(
                crate::https_url_text_ref::HttpsUrlTextRef::from(valid_port.as_str()),
            ),
            Ok(())
        );
        assert!([invalid_port, whitespace].iter().all(|value| {
            crate::validate_https_url_text::validate_https_url_text(
                crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
            ) == Err(crate::https_url_text_error::HttpsUrlTextError::Invalid)
        }));
    }

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
    #[test]
    fn test_url_safe_token_limit_is_inclusive_and_precedes_symbol_validation() {
        [
            (
                constants_str::ABC_ALT_3,
                constants_str::ABC_ALT_3.len(),
                Ok(()),
            ),
            (
                constants_str::EMPTY,
                constants_usize::ZERO,
                Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::Empty),
            ),
            (
                constants_str::TEST_URL_TOKEN_WITH_SEPARATOR,
                constants_usize::ZERO,
                Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong),
            ),
            (
                constants_str::SPACE,
                constants_usize::ONE,
                Err(
                    crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::InvalidSymbol,
                ),
            ),
        ]
        .into_iter()
        .fold((), |(), (value, maximum, expected)| {
            assert_eq!(
                crate::validate_url_safe_token_part::validate_url_safe_token_part(
                    crate::url_safe_token_part_ref::UrlSafeTokenPartRef::from(value),
                    crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                        maximum
                    ),
                ),
                expected,
            );
        });
    }

    #[test]
    fn test_https_url_rejects_control_characters_and_backslashes_in_path() {
        ['\0', '\t', '\n', '\r', '\\', '\u{7f}']
            .into_iter()
            .fold((), |(), character| {
                let mut value = constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH.to_owned();
                value.push(character);
                assert_eq!(
                    crate::validate_https_url_text::validate_https_url_text(
                        crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
                    ),
                    Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
                );
            });
    }

    #[test]
    fn test_https_url_rejects_empty_and_zero_ports() {
        [constants_str::EMPTY, constants_str::VALUE_0]
            .into_iter()
            .fold((), |(), port| {
                let value = format!("{}:{port}", constants_str::HTTPS_ADMIN_EXAMPLE_COM);
                assert_eq!(
                    crate::validate_https_url_text::validate_https_url_text(
                        crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
                    ),
                    Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
                );
            });
    }
    #[test]
    fn test_trimmed_text_preserves_internal_spaces_and_rejects_empty_or_nul() {
        let padded = format!(" {} {} ", constants_str::X, constants_str::X);
        let trimmed_expected = format!("{} {}", constants_str::X, constants_str::X);
        assert!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(padded)
                .is_ok_and(|value| value.as_ref() == trimmed_expected)
        );
        [
            (
                constants_str::EMPTY,
                crate::bounded_text_policy_error::BoundedTextPolicyError::Empty,
            ),
            (
                constants_str::SPACE,
                crate::bounded_text_policy_error::BoundedTextPolicyError::Empty,
            ),
            (
                constants_str::TEST_TEXT_WITH_NUL,
                crate::bounded_text_policy_error::BoundedTextPolicyError::ContainsNul,
            ),
        ]
        .into_iter()
        .fold((), |(), (value, expected)| {
            assert_eq!(
                crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(value.to_owned()),
                Err(expected)
            );
        });
    }

    #[test]
    fn test_bounded_text_wrappers_accept_maximum_and_reject_oversized_input_before_trimming() {
        let maximum = constants_str::X.repeat(constants_usize::VALUE_1_048_576);
        assert!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(maximum.clone())
                .is_ok_and(|value| value.as_ref() == maximum)
        );
        assert!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                maximum.clone()
            )
            .is_ok_and(|value| value.as_ref() == maximum)
        );
        let oversized = format!("{maximum}{}", constants_str::SPACE);
        assert_eq!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(oversized.clone()),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::TooLong)
        );
        assert_eq!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(oversized),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::TooLong)
        );
    }

    #[test]
    fn test_required_text_preserves_whitespace_and_rejects_empty() {
        assert!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                constants_str::SPACE.to_owned()
            )
            .is_ok_and(|value| value.as_ref() == constants_str::SPACE)
        );
        assert_eq!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(
                constants_str::EMPTY.to_owned()
            ),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::Empty)
        );
    }

    #[test]
    fn test_fixed_hex_rejects_wrong_lengths_before_invalid_symbols() {
        let length = constants_str::TEST_GIT_COMMIT_HASH.len();
        [constants_usize::ZERO, length - constants_usize::ONE, length + constants_usize::ONE]
            .into_iter().fold((), |(), invalid_length| {
                assert_eq!(crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(constants_str::X.repeat(invalid_length)), Err(crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidLength));
            });
        assert_eq!(crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(constants_str::X.repeat(length)), Err(crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidSymbol));
    }
}
