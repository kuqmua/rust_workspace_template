#[cfg(test)]
mod tests {
    #[test]
    fn test_builder_joins_validated_directives() {
        let mut builder = crate::http_csp_builder::HttpCspBuilder::default();
        let default_src = crate::http_csp_directive_name::HttpCspDirectiveName::try_from(
            String::from(constants_str::TEST_DEFAULT_SRC),
        )
        .expect("e692ea17 builder_joins_validated_directives invariant must hold");
        let self_value = crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(
            String::from(constants_str::TEST_CSP_SELF),
        )
        .expect("ca342c81 builder_joins_validated_directives invariant must hold");
        builder
            .try_add(&default_src, &[self_value])
            .expect("6d089fc9 builder_joins_validated_directives invariant must hold");
        let policy = builder
            .try_build()
            .expect("1a987236 builder_joins_validated_directives invariant must hold");
        assert_eq!(
            policy
                .to_str()
                .expect("ba8ae30f builder_joins_validated_directives invariant must hold"),
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

// Root-owned module compatibility wrappers.
