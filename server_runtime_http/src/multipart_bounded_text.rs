#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub(crate) struct MultipartBoundedText<const MAXIMUM_LEN: usize>(
    bounded_types::bounded_string::BoundedString<0usize, MAXIMUM_LEN, false>,
);

impl<const MAXIMUM_LEN: usize> TryFrom<String> for MultipartBoundedText<MAXIMUM_LEN> {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > MAXIMUM_LEN {
            return Err(Self::Error::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(value.len()),
            });
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    ..
                } => Self::Error::TooLong {
                    actual: crate::multipart_value_length::MultipartValueLength::from(
                        actual_length.get(),
                    ),
                },
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multipart_bounded_text_preserves_content_and_counts_utf8_bytes() {
        let unicode = '\u{e9}'.to_string().repeat(2usize);
        let mut oversized_unicode = unicode.clone();
        oversized_unicode.push('x');
        assert!(
            [
                String::new(),
                [' ', '\t', '\n', ' '].into_iter().collect::<String>(),
                constants_str::X.repeat(3usize),
                constants_str::X.repeat(4usize),
                constants_str::X.repeat(5usize),
                unicode,
                oversized_unicode,
            ]
            .into_iter()
            .all(|text| {
                let result =
                    crate::multipart_bounded_text::MultipartBoundedText::<4usize>::try_from(
                        text.clone(),
                    );
                if text.len() <= 4usize {
                    result.is_ok_and(|value| value.as_ref() == text)
                } else {
                    result
                        == Err(crate::multipart_value_error::MultipartValueError::TooLong {
                            actual: crate::multipart_value_length::MultipartValueLength::from(
                                text.len(),
                            ),
                        })
                }
            })
        );
        assert!(
            crate::multipart_bounded_text::MultipartBoundedText::<0usize>::try_from(String::new())
                .is_ok_and(|value| value.as_ref().is_empty())
        );
        assert_eq!(
            crate::multipart_bounded_text::MultipartBoundedText::<0usize>::try_from(
                constants_str::X.to_owned()
            ),
            Err(crate::multipart_value_error::MultipartValueError::TooLong {
                actual: crate::multipart_value_length::MultipartValueLength::from(1usize),
            })
        );
    }
}
