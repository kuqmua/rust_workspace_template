#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveValue(String);

impl HttpCspDirectiveValue {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpCspDirectiveValue {
    type Error = crate::http_csp_token_error::HttpCspTokenError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err(crate::http_csp_token_error::HttpCspTokenError::Empty);
        }
        if string.len() > constants_usize::VALUE_1_024 {
            return Err(crate::http_csp_token_error::HttpCspTokenError::TooLong);
        }
        if string
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte == b';')
        {
            return Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(string))
    }
}
