#[cfg(test)]
mod tests {
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
