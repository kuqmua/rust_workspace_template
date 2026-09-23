#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(crate) struct HttpCspTokenText<const MAXIMUM_LEN: usize>(
    bounded_types::bounded_string::BoundedString<1usize, MAXIMUM_LEN, false>,
);

impl<const MAXIMUM_LEN: usize> HttpCspTokenText<MAXIMUM_LEN> {
    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<const MAXIMUM_LEN: usize> TryFrom<String> for HttpCspTokenText<MAXIMUM_LEN> {
    type Error = crate::http_csp_token_error::HttpCspTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Empty);
        }
        if value.len() > MAXIMUM_LEN {
            return Err(Self::Error::TooLong);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                } => Self::Error::TooLong,
                bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::Empty,
            })
    }
}
