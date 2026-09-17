#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub(super) struct HttpOriginSchemeText(
    bounded_types::bounded_string::BoundedString<1usize, 16usize, false>,
);

impl HttpOriginSchemeText {
    pub(crate) const fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for HttpOriginSchemeText {
    type Error = crate::allowed_origin_error::AllowedOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 16usize {
            Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Invalid,
                })
        }
    }
}
