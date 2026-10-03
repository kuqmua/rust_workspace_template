#[cfg(test)]
mod tests {
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
