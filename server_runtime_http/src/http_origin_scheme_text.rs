#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(super) struct HttpOriginSchemeText(String);

impl HttpOriginSchemeText {
    pub(crate) const fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpOriginSchemeText {
    type Error = crate::allowed_origin_error::AllowedOriginError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() || string.len() > 16usize {
            Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
        } else {
            Ok(Self(string))
        }
    }
}
