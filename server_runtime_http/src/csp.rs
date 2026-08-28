#[path = "http_csp_builder.rs"]
mod http_csp_builder;
#[path = "http_csp_directive_name.rs"]
mod http_csp_directive_name;
#[path = "http_csp_directive_value.rs"]
mod http_csp_directive_value;
#[path = "http_csp_maximum_bytes_error.rs"]
mod http_csp_maximum_bytes_error;
#[path = "http_csp_token_error.rs"]
mod http_csp_token_error;

pub use http_csp_builder::HttpCspBuilder;
pub use http_csp_directive_name::HttpCspDirectiveName;
pub use http_csp_directive_value::HttpCspDirectiveValue;
pub use http_csp_maximum_bytes_error::HttpCspMaximumBytesError;
pub use http_csp_token_error::HttpCspTokenError;

#[cfg(test)]
mod tests {
    #[test]
    fn builder_joins_validated_directives() {
        let mut builder = super::HttpCspBuilder::default();
        let default_src =
            super::HttpCspDirectiveName::try_from(String::from(constants_str::TEST_DEFAULT_SRC))
                .expect("e692ea17 builder_joins_validated_directives invariant must hold");
        let self_value =
            super::HttpCspDirectiveValue::try_from(String::from(constants_str::TEST_CSP_SELF))
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
    fn tokens_reject_whitespace_semicolon_and_uppercase_name() {
        assert_eq!(
            super::HttpCspDirectiveValue::try_from(String::from(constants_str::TEST_CSP_SELF_DATA)),
            Err(super::HttpCspTokenError::InvalidCharacter)
        );
        assert_eq!(
            super::HttpCspDirectiveValue::try_from(String::from(constants_str::TEST_CSP_DATA_SEMI)),
            Err(super::HttpCspTokenError::InvalidCharacter)
        );
        assert_eq!(
            super::HttpCspDirectiveName::try_from(String::from(
                constants_str::TEST_DEFAULT_SRC_UPPER
            )),
            Err(super::HttpCspTokenError::InvalidCharacter)
        );
    }
}
