#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct HttpNormalizedPath(
    bounded_types::bounded_string::BoundedString<0usize, 8_192usize, false>,
);

impl TryFrom<String> for HttpNormalizedPath {
    type Error = crate::http_normalized_path_error::HttpNormalizedPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len().cmp(&constants_usize::VALUE_8_192) {
            std::cmp::Ordering::Greater => {
                Err(crate::http_normalized_path_error::HttpNormalizedPathError::TooLarge)
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                bounded_types::bounded_string::BoundedString::try_from(value)
                    .map(Self)
                    .map_err(|source| match source {
                        bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                            ..
                        }
                        | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                            ..
                        } => Self::Error::TooLarge,
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_normalized_path_preserves_text_at_ascii_and_utf8_limits() {
        assert!(
            [
                String::default(),
                constants_str::X.to_owned(),
                constants_str::X.repeat(8191usize),
                constants_str::X.repeat(8192usize),
                constants_str::X.repeat(8193usize),
                '\u{e9}'.to_string().repeat(4096usize),
                format!(
                    "{}{}",
                    '\u{e9}'.to_string().repeat(4096usize),
                    constants_str::X
                ),
            ]
            .into_iter()
            .all(|text| {
                let valid = text.len() <= 8192usize;
                let result =
                    crate::http_normalized_path::HttpNormalizedPath::try_from(text.clone());
                if valid {
                    result.is_ok_and(|http_normalized_path| http_normalized_path.as_ref() == text)
                } else {
                    result
                        == Err(crate::http_normalized_path_error::HttpNormalizedPathError::TooLarge)
                }
            })
        );
    }
}
