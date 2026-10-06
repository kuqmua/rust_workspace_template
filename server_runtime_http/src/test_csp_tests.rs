#[cfg(test)]
mod tests {
    #[test]
    fn test_csp_token_storage_zero_and_utf8_limits_preserve_validation_order() {
        assert_eq!(
            crate::http_csp_token_text::HttpCspTokenText::<0usize>::try_from(String::new()),
            Err(crate::http_csp_token_error::HttpCspTokenError::Empty)
        );
        assert_eq!(
            crate::http_csp_token_text::HttpCspTokenText::<0usize>::try_from(
                constants_str::X.to_owned()
            ),
            Err(crate::http_csp_token_error::HttpCspTokenError::TooLong)
        );
        assert_eq!(
            crate::http_csp_token_text::HttpCspTokenText::<1usize>::try_from(
                '\u{00e9}'.to_string()
            ),
            Err(crate::http_csp_token_error::HttpCspTokenError::TooLong)
        );
        let input = '\u{00e9}'.to_string();
        let pointer = input.as_ptr();
        assert!(
            crate::http_csp_token_text::HttpCspTokenText::<2usize>::try_from(input).is_ok_and(
                |http_csp_token_text| {
                    http_csp_token_text.as_str().len() == 2usize
                        && http_csp_token_text.as_str().as_ptr() == pointer
                        && http_csp_token_text
                            .as_str()
                            .chars()
                            .eq(std::iter::once('\u{00e9}'))
                }
            )
        );
        assert!(
            crate::http_csp_token_text::HttpCspTokenText::<1usize>::try_from('\0'.to_string())
                .is_ok_and(|http_csp_token_text| {
                    http_csp_token_text.as_str().as_bytes() == [0u8]
                })
        );
    }

    #[test]
    fn test_csp_directive_printable_ascii_character_rules() {
        assert!((32u8..=126u8).all(|byte| {
            let text = char::from(byte).to_string();
            let name = crate::http_csp_directive_name::HttpCspDirectiveName::try_from(text.clone());
            let value =
                crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(text.clone());
            let name_matches = if byte.is_ascii_lowercase() || byte == b'-' {
                name.is_ok_and(|http_csp_directive_name| http_csp_directive_name.as_str() == text)
            } else {
                name == Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter)
            };
            let value_matches = if matches!(byte, b' ' | b';') {
                value == Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter)
            } else {
                value
                    .is_ok_and(|http_csp_directive_value| http_csp_directive_value.as_str() == text)
            };
            name_matches && value_matches
        }));
    }

    #[test]
    fn test_csp_directive_exact_size_limits_and_error_precedence() {
        assert!(
            [0usize, 1usize, 63usize, 64usize, 65usize]
                .into_iter()
                .all(|length| {
                    let text = constants_str::X.repeat(length);
                    let result = crate::http_csp_directive_name::HttpCspDirectiveName::try_from(
                        text.clone(),
                    );
                    match length {
                        0usize => {
                            result == Err(crate::http_csp_token_error::HttpCspTokenError::Empty)
                        }
                        65usize => {
                            result == Err(crate::http_csp_token_error::HttpCspTokenError::TooLong)
                        }
                        _ => result.is_ok_and(|http_csp_directive_name| {
                            http_csp_directive_name.as_str() == text
                        }),
                    }
                })
        );
        assert!(
            [0usize, 1usize, 1023usize, 1024usize, 1025usize]
                .into_iter()
                .all(|length| {
                    let text = constants_str::X.repeat(length);
                    let result = crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(
                        text.clone(),
                    );
                    match length {
                        0usize => {
                            result == Err(crate::http_csp_token_error::HttpCspTokenError::Empty)
                        }
                        1025usize => {
                            result == Err(crate::http_csp_token_error::HttpCspTokenError::TooLong)
                        }
                        _ => result.is_ok_and(|http_csp_directive_value| {
                            http_csp_directive_value.as_str() == text
                        }),
                    }
                })
        );
        assert_eq!(
            crate::http_csp_directive_name::HttpCspDirectiveName::try_from(
                'A'.to_string().repeat(65usize)
            ),
            Err(crate::http_csp_token_error::HttpCspTokenError::TooLong)
        );
        assert_eq!(
            crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(
                ';'.to_string().repeat(1025usize)
            ),
            Err(crate::http_csp_token_error::HttpCspTokenError::TooLong)
        );
    }

    #[test]
    fn test_csp_builder_preserves_multiple_value_order_and_directive_separators() {
        let name = crate::http_csp_directive_name::HttpCspDirectiveName::try_from(
            constants_str::TEST_DEFAULT_SRC.to_owned(),
        );
        assert!(name.is_ok_and(|http_csp_directive_name| {
            let values = [constants_str::TEST_CSP_SELF, constants_str::X].map(|value| {
                crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(value.to_owned())
            });
            let [Ok(first_value), Ok(second_value)] = values else {
                return false;
            };
            let expected = [
                constants_str::TEST_DEFAULT_SRC,
                constants_str::SPACE,
                constants_str::TEST_CSP_SELF,
                constants_str::SPACE,
                constants_str::X,
                constants_str::HTTP_CSP_DIRECTIVE_SEPARATOR,
                constants_str::TEST_DEFAULT_SRC,
            ]
            .concat();
            let mut builder = crate::http_csp_builder::HttpCspBuilder::default();
            builder
                .try_add(&http_csp_directive_name, &[first_value, second_value])
                .is_ok()
                && builder.try_add(&http_csp_directive_name, &[]).is_ok()
                && builder
                    .try_build()
                    .is_ok_and(|http_content_security_policy| {
                        http_content_security_policy.as_bytes() == expected.as_bytes()
                    })
        }));
    }

    #[test]
    fn test_builder_joins_validated_directives() {
        let mut builder = crate::http_csp_builder::HttpCspBuilder::default();
        let default_src = crate::http_csp_directive_name::HttpCspDirectiveName::try_from(
            String::from(constants_str::TEST_DEFAULT_SRC),
        )
        .expect(constants_str::DIAGNOSTIC_E692EA17);
        let self_value = crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(
            String::from(constants_str::TEST_CSP_SELF),
        )
        .expect(constants_str::DIAGNOSTIC_CA342C81);
        builder
            .try_add(&default_src, &[self_value])
            .expect(constants_str::DIAGNOSTIC_6D089FC9);
        let policy = builder
            .try_build()
            .expect(constants_str::DIAGNOSTIC_1A987236);
        assert_eq!(
            policy.to_str().expect(constants_str::DIAGNOSTIC_BA8AE30F),
            constants_str::TEST_DEFAULT_SRC_SELF
        );
    }

    #[test]
    fn test_tokens_reject_whitespace_semicolon_and_uppercase_name() {
        assert_eq!(
            crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(String::from(
                constants_str::TEST_CSP_SELF_DATA
            )),
            Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter)
        );
        assert_eq!(
            crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(String::from(
                constants_str::TEST_CSP_DATA_SEMI
            )),
            Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter)
        );
        assert_eq!(
            crate::http_csp_directive_name::HttpCspDirectiveName::try_from(String::from(
                constants_str::TEST_DEFAULT_SRC_UPPER
            )),
            Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter)
        );
    }
}
