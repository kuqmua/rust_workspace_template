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

#[cfg(test)]
mod tests {
    #[test]
    fn test_origin_scheme_text_exact_ascii_and_utf8_limits() {
        assert!(
            [
                String::default(),
                constants_str::X.to_owned(),
                constants_str::X.repeat(15usize),
                constants_str::X.repeat(16usize),
                constants_str::X.repeat(17usize),
                '\u{e9}'.to_string().repeat(8usize),
                format!(
                    "{}{}",
                    '\u{e9}'.to_string().repeat(8usize),
                    constants_str::X
                ),
            ]
            .into_iter()
            .all(|text| {
                let valid = (1usize..=16usize).contains(&text.len());
                let result =
                    crate::http_origin_scheme_text::HttpOriginSchemeText::try_from(text.clone());
                if valid {
                    result
                        .is_ok_and(|http_origin_scheme_text| http_origin_scheme_text.get() == text)
                } else {
                    result == Err(crate::allowed_origin_error::AllowedOriginError::Invalid)
                }
            })
        );
    }
}
