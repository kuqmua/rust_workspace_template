#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpCspDirectiveName(
    crate::http_csp_token_text::HttpCspTokenText<{ constants_usize::VALUE_64 }>,
);

impl HttpCspDirectiveName {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpCspDirectiveName {
    type Error = crate::http_csp_token_error::HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let http_csp_token_text = crate::http_csp_token_text::HttpCspTokenText::try_from(value)?;
        if !http_csp_token_text
            .as_str()
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        {
            return Err(crate::http_csp_token_error::HttpCspTokenError::InvalidCharacter);
        }
        Ok(Self(http_csp_token_text))
    }
}
