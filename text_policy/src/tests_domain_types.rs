#[cfg(test)]
mod tests {
    #[test]
    fn test_password_text_reference_preserves_borrowing_and_exact_debug_redaction() {
        [
            constants_str::EMPTY,
            constants_str::NON_ASCII_U_E9,
            constants_str::NEVER_PRINT_THIS_VALUE,
        ]
        .into_iter()
        .fold((), |(), value| {
            let password_text_ref = crate::password_text_ref::PasswordTextRef::from(value);
            assert_eq!(
                format!("{password_text_ref:?}"),
                constants_str::REDACTED_ALT_3
            );
            assert_eq!(
                format!("{password_text_ref:#?}"),
                constants_str::REDACTED_ALT_3
            );
            let borrowed = <&str>::from(password_text_ref);
            assert!(std::ptr::eq(borrowed, value));
            assert_eq!(
                format!("{password_text_ref:?}"),
                constants_str::REDACTED_ALT_3
            );
        });
    }

    #[test]
    fn test_https_url_requires_exact_lowercase_scheme_prefix() {
        let valid = constants_str::HTTPS_ADMIN_EXAMPLE_COM;
        let authority = valid.trim_start_matches(constants_str::HTTPS_SCHEME_PREFIX);
        [authority.to_owned(), valid.to_uppercase(), String::new()]
            .iter()
            .fold((), |(), value| {
                assert_eq!(
                    crate::validate_https_url_text::validate_https_url_text(
                        crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
                    ),
                    Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
                );
            });
    }

    #[test]
    fn test_https_url_rejects_unicode_whitespace_in_path_query_and_fragment() {
        ['\u{00a0}', '\u{2003}', '\u{2028}', '\u{202f}', '\u{3000}']
            .into_iter()
            .fold((), |(), character| {
                ['/', '?', '#'].into_iter().fold((), |(), delimiter| {
                    let mut value = constants_str::HTTPS_ADMIN_EXAMPLE_COM.to_owned();
                    value.push(delimiter);
                    value.push(character);
                    assert_eq!(
                        crate::validate_https_url_text::validate_https_url_text(
                            crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
                        ),
                        Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
                    );
                });
            });
    }

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
    fn test_fixed_hex_ascii_alphabet_preserves_owned_content_and_utf8_precedence() {
        let length = constants_str::TEST_GIT_COMMIT_HASH.len();
        assert!((0u8..=127u8).all(|byte| {
            let character = char::from(byte);
            let input = character.to_string().repeat(length);
            let pointer = input.as_ptr();
            let accepted = constants_str::ASCII_UPPER_HEX_DIGITS.iter()
                .any(|digit| digit.to_ascii_lowercase() == byte);
            let result = crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(input);
            if accepted {
                result.is_ok_and(|fixed_length_ascii_hex_text| {
                    fixed_length_ascii_hex_text.as_ref().len() == length
                        && fixed_length_ascii_hex_text.as_ref().as_ptr() == pointer
                        && fixed_length_ascii_hex_text.as_ref().chars().all(|value| value == character)
                })
            } else {
                result.is_err_and(|error| error == crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidSymbol)
            }
        }));
        let unicode_character = '\u{00e9}';
        let unicode_text = unicode_character
            .to_string()
            .repeat(length.div_euclid(unicode_character.len_utf8()));
        assert_eq!(unicode_text.len(), length);
        assert_eq!(
            crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(unicode_text),
            Err(crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidSymbol)
        );
        assert_eq!(
            crate::fixed_length_ascii_hex_text::FixedLengthAsciiHexText::try_from(unicode_character.to_string().repeat(length.div_euclid(unicode_character.len_utf8()) + 1usize)),
            Err(crate::fixed_length_ascii_hex_text_error::FixedLengthAsciiHexTextError::InvalidLength)
        );
    }

    #[test]
    fn test_owned_url_token_parts_enforce_alphabet_limits_and_error_precedence() {
        assert!((0u8..=127u8).all(|byte| {
            let value = char::from(byte).to_string();
            let pointer = value.as_ptr();
            let result = crate::url_safe_token_part_text::UrlSafeTokenPartText::try_from(value);
            if char::from(byte).is_alphanumeric()
                || matches!(byte, b'-' | b'_')
            {
                result.is_ok_and(|url_safe_token_part_text| {
                    url_safe_token_part_text.as_ref().as_bytes() == [byte]
                        && url_safe_token_part_text.as_ref().as_ptr() == pointer
                })
            } else {
                result == Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::InvalidSymbol)
            }
        }));
        let maximum = crate::url_safe_token_part_maximum_bytes::URL_SAFE_TOKEN_PART_MAXIMUM_BYTES;
        let maximum_value = constants_str::X.repeat(maximum);
        let pointer = maximum_value.as_ptr();
        assert!(
            crate::url_safe_token_part_text::UrlSafeTokenPartText::try_from(maximum_value)
                .is_ok_and(|url_safe_token_part_text| {
                    url_safe_token_part_text.as_ref().len() == maximum
                        && url_safe_token_part_text.as_ref().as_ptr() == pointer
                        && url_safe_token_part_text
                            .as_ref()
                            .chars()
                            .all(|character| character == 'x')
                })
        );
        [
            (
                String::new(),
                crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::Empty,
            ),
            (
                '\u{e9}'.to_string(),
                crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::InvalidSymbol,
            ),
            (
                constants_str::X.repeat(maximum + 1usize),
                crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong,
            ),
            (
                '?'.to_string().repeat(maximum + 1usize),
                crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong,
            ),
        ]
        .into_iter()
        .fold((), |(), (value, expected)| {
            assert_eq!(
                crate::url_safe_token_part_text::UrlSafeTokenPartText::try_from(value),
                Err(expected)
            );
        });
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
    fn test_password_policy_missing_character_classes_and_error_precedence() {
        let range = crate::password_length_range::PasswordLengthRange::from_prevalidated(
            crate::password_length::PasswordLength::from(0usize),
            crate::password_length::PasswordLength::from(128usize),
        );
        assert!(
            [
                (
                    true,
                    false,
                    false,
                    false,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingDigit
                ),
                (
                    false,
                    true,
                    false,
                    false,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingLowercase
                ),
                (
                    false,
                    false,
                    true,
                    false,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingUppercase
                ),
                (
                    false,
                    false,
                    false,
                    true,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingSpecial
                ),
                (
                    true,
                    true,
                    true,
                    true,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingDigit
                ),
                (
                    false,
                    true,
                    true,
                    true,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingLowercase
                ),
                (
                    false,
                    false,
                    true,
                    true,
                    crate::password_policy_violation::PasswordPolicyViolation::MissingUppercase
                ),
            ]
            .into_iter()
            .all(
                |(remove_digit, remove_lowercase, remove_uppercase, remove_special, expected)| {
                    let password = constants_str::TEST_STRONG_PASSWORD
                        .chars()
                        .filter(|character| {
                            !(remove_digit && character.is_ascii_digit()
                                || remove_lowercase && character.is_ascii_lowercase()
                                || remove_uppercase && character.is_ascii_uppercase()
                                || remove_special && character.is_ascii_punctuation())
                        })
                        .collect::<String>();
                    crate::validate_password_policy::validate_password_policy(
                        crate::password_text_ref::PasswordTextRef::from(password.as_str()),
                        range,
                    ) == Err(expected)
                }
            )
        );
    }

    #[test]
    fn test_password_policy_maximum_length_and_length_error_precedence() {
        let password_length = constants_str::TEST_STRONG_PASSWORD.chars().count();
        let range = crate::password_length_range::PasswordLengthRange::from_prevalidated(
            crate::password_length::PasswordLength::from(password_length),
            crate::password_length::PasswordLength::from(password_length),
        );
        let excessive_password = constants_str::TEST_STRONG_PASSWORD
            .chars()
            .chain(char::from_u32(0x430u32))
            .collect::<String>();
        let excessive_whitespace = constants_str::SPACE.repeat(password_length + 1usize);
        assert!(
            [
                (constants_str::TEST_STRONG_PASSWORD, Ok(())),
                (
                    excessive_password.as_str(),
                    Err(crate::password_policy_violation::PasswordPolicyViolation::TooLong)
                ),
                (
                    excessive_whitespace.as_str(),
                    Err(crate::password_policy_violation::PasswordPolicyViolation::TooLong)
                ),
                (
                    constants_str::SPACE,
                    Err(crate::password_policy_violation::PasswordPolicyViolation::TooShort)
                ),
            ]
            .into_iter()
            .all(|(password, expected)| {
                crate::validate_password_policy::validate_password_policy(
                    crate::password_text_ref::PasswordTextRef::from(password),
                    range,
                ) == expected
            })
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
    fn test_trimmed_text_unicode_edges_and_raw_byte_limit_preserve_validation_order() {
        let content = format!("{}{}{}", '\u{e9}', '\u{a0}', '\u{e9}');
        let padded = format!("{}{}{}", '\u{a0}', content, '\u{2003}');
        assert!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(padded)
                .is_ok_and(|non_empty_trimmed_text| non_empty_trimmed_text.as_ref() == content)
        );
        assert_eq!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(format!(
                "{}{}",
                '\u{a0}', '\u{2003}'
            )),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::Empty),
        );
        let maximum = constants_usize::VALUE_1_048_576;
        let boundary_content = '\u{e9}'
            .to_string()
            .repeat((maximum - 4usize).div_euclid(2usize));
        let exact = format!("{}{}{}", '\u{a0}', boundary_content, '\u{a0}');
        assert!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(exact).is_ok_and(
                |non_empty_trimmed_text| non_empty_trimmed_text.as_ref() == boundary_content
            )
        );
        let oversized = format!(
            "{}{}{}",
            '\u{a0}',
            '\u{e9}'.to_string().repeat(maximum.div_euclid(2usize)),
            '\0'
        );
        assert_eq!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(oversized),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::TooLong),
        );
        assert_eq!(
            crate::non_empty_trimmed_text::NonEmptyTrimmedText::try_from(format!(
                "{}{}{}",
                '\u{a0}', '\0', '\u{2003}'
            )),
            Err(crate::bounded_text_policy_error::BoundedTextPolicyError::ContainsNul),
        );
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
    fn test_nul_free_text_utf8_byte_limits_preserve_content_and_error_precedence() {
        let maximum = constants_usize::VALUE_1_048_576;
        let exact = '\u{e9}'.to_string().repeat(maximum.div_euclid(2usize));
        let pointer = exact.as_ptr();
        assert!(
            crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(exact)
                .is_ok_and(|required_nul_free_bounded_text| {
                    required_nul_free_bounded_text.as_ref().len() == maximum
                        && required_nul_free_bounded_text.as_ref().as_ptr() == pointer
                        && required_nul_free_bounded_text
                            .as_ref()
                            .chars()
                            .all(|character| character == '\u{e9}')
                })
        );
        [
            (
                maximum - 2usize,
                crate::bounded_text_policy_error::BoundedTextPolicyError::ContainsNul,
            ),
            (
                maximum,
                crate::bounded_text_policy_error::BoundedTextPolicyError::TooLong,
            ),
        ]
        .into_iter()
        .fold((), |(), (length, expected)| {
            let mut value = '\u{e9}'.to_string().repeat(length.div_euclid(2usize));
            value.push('\0');
            assert_eq!(
                crate::required_nul_free_bounded_text::RequiredNulFreeBoundedText::try_from(value),
                Err(expected),
            );
        });
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

    #[test]
    fn test_https_url_host_labels_reject_empty_and_edge_hyphens() {
        let base = constants_str::HTTPS_ADMIN_EXAMPLE_COM;
        let authority = base.trim_start_matches(constants_str::HTTPS_SCHEME_PREFIX);
        let invalid_authorities = [
            String::new(),
            constants_str::X.to_owned(),
            format!(".{authority}"),
            format!("{authority}."),
            format!("{authority}..{authority}"),
            format!("-{authority}"),
            format!("{authority}-"),
        ];
        invalid_authorities
            .iter()
            .fold((), |(), invalid_authority| {
                let value = format!("{}{invalid_authority}", constants_str::HTTPS_SCHEME_PREFIX);
                assert_eq!(
                    crate::validate_https_url_text::validate_https_url_text(
                        crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
                    ),
                    Err(crate::https_url_text_error::HttpsUrlTextError::Invalid),
                );
            });
        ['?', '#'].into_iter().fold((), |(), delimiter| {
            let value = format!("{base}{delimiter}{}", constants_str::X);
            assert_eq!(
                crate::validate_https_url_text::validate_https_url_text(
                    crate::https_url_text_ref::HttpsUrlTextRef::from(value.as_str()),
                ),
                Ok(()),
            );
        });
    }

    #[test]
    fn test_password_length_range_conversion_preserves_inclusive_bounds() {
        [(0usize, 0usize), (12usize, 12usize), (12usize, 128usize)]
            .into_iter()
            .fold((), |(), (minimum_length, maximum_length)| {
                let minimum = crate::password_length::PasswordLength::from(minimum_length);
                let maximum = crate::password_length::PasswordLength::from(maximum_length);
                let result = crate::password_length_range::PasswordLengthRange::try_from((minimum, maximum));
                assert!(result.is_ok_and(|range| range.minimum() == minimum && range.maximum() == maximum));
            });
        assert_eq!(
            crate::password_length_range::PasswordLengthRange::try_from((
                crate::password_length::PasswordLength::from(13usize),
                crate::password_length::PasswordLength::from(12usize),
            )),
            Err(crate::password_length_range_error::PasswordLengthRangeError::Invalid),
        );
    }
}
