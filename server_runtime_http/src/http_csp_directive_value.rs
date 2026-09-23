#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveValue(
    crate::http_csp_token_text::HttpCspTokenText<{ constants_usize::VALUE_1_024 }>,
);

impl HttpCspDirectiveValue {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpCspDirectiveValue {
    type Error = crate::http_csp_token_error::HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let http_csp_token_text = crate::http_csp_token_text::HttpCspTokenText::try_from(value)?;
        if http_csp_token_text
            .as_str()
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte == b';')
        {
            return Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(http_csp_token_text))
    }
}
