const HTTP_CSP_MAXIMUM_BYTES: usize = 4096usize;
const HTTP_CSP_NAME_MAXIMUM_BYTES: usize = 64usize;
const HTTP_CSP_VALUE_MAXIMUM_BYTES: usize = 1024usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveName(String);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveValue(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpCspTokenError {
    #[error("content security policy token must not be empty")]
    Empty,
    #[error("content security policy token contains an invalid character")]
    InvalidCharacter,
    #[error("content security policy token is too long")]
    TooLong,
}

impl TryFrom<String> for HttpCspDirectiveName {
    type Error = HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(HttpCspTokenError::Empty);
        }
        if value.len() > HTTP_CSP_NAME_MAXIMUM_BYTES {
            return Err(HttpCspTokenError::TooLong);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        {
            return Err(HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(value))
    }
}

impl TryFrom<String> for HttpCspDirectiveValue {
    type Error = HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(HttpCspTokenError::Empty);
        }
        if value.len() > HTTP_CSP_VALUE_MAXIMUM_BYTES {
            return Err(HttpCspTokenError::TooLong);
        }
        if value
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte == b';')
        {
            return Err(HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(value))
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq)]
pub struct HttpCspBuilder(String);

impl TryFrom<String> for HttpCspBuilder {
    type Error = HttpCspMaximumBytesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > HTTP_CSP_MAXIMUM_BYTES {
            return Err(HttpCspMaximumBytesError);
        }
        Ok(Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("content security policy exceeds 4096 bytes")]
pub struct HttpCspMaximumBytesError;

impl HttpCspBuilder {
    pub fn try_add(
        &mut self,
        name: &HttpCspDirectiveName,
        values: &[HttpCspDirectiveValue],
    ) -> Result<(), HttpCspMaximumBytesError> {
        let separator_bytes = if self.0.is_empty() {
            constants_usize::ZERO
        } else {
            2usize
        };
        let values_bytes = values
            .iter()
            .map(|value| value.0.len().saturating_add(constants_usize::ONE))
            .sum::<usize>();
        let added_bytes = separator_bytes
            .saturating_add(name.0.len())
            .saturating_add(values_bytes);
        if self.0.len().saturating_add(added_bytes) > HTTP_CSP_MAXIMUM_BYTES {
            return Err(HttpCspMaximumBytesError);
        }
        self.0.reserve(added_bytes);
        if !self.0.is_empty() {
            self.0.push_str(constants_str::HTTP_CSP_DIRECTIVE_SEPARATOR);
        }
        self.0.push_str(name.0.as_str());
        let _text = values.iter().fold(&mut self.0, |text, value| {
            text.push(' ');
            text.push_str(value.0.as_str());
            text
        });
        Ok(())
    }

    pub fn try_build(
        self,
    ) -> Result<
        crate::domain_types::HttpContentSecurityPolicy,
        crate::domain_types::HttpContentSecurityPolicyError,
    > {
        crate::domain_types::HttpContentSecurityPolicy::try_from(self.0)
    }
}

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
