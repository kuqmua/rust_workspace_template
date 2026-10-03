#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct StoragePathSegment(
    bounded_types::bounded_string::BoundedString<1usize, 1_024usize, false>,
);

impl TryFrom<String> for StoragePathSegment {
    type Error = crate::storage_path_segment_error::StoragePathSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_1_024 {
            return Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid);
        }
        text_policy::validate_url_safe_token_part::validate_url_safe_token_part(
            text_policy::url_safe_token_part_ref::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                constants_usize::VALUE_1_024,
            ),
        )
        .map_err(|_error| crate::storage_path_segment_error::StoragePathSegmentError::Invalid)?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_storage_segment_ascii_alphabet_and_text_preservation() {
        assert!((0u8..=127u8).all(|byte| {
            let text = char::from(byte).to_string();
            let result = crate::storage_path_segment::StoragePathSegment::try_from(text.clone());
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_') {
                result.is_ok_and(|storage_path_segment| storage_path_segment.as_ref() == text)
            } else {
                result == Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
            }
        }));
        assert_eq!(
            crate::storage_path_segment::StoragePathSegment::try_from('\u{e9}'.to_string()),
            Err(crate::storage_path_segment_error::StoragePathSegmentError::Invalid)
        );
    }

    #[test]
    fn test_storage_segment_minimum_and_exact_maximum_lengths() {
        assert!(
            [0usize, 1usize, 1023usize, 1024usize, 1025usize]
                .into_iter()
                .all(|length| {
                    let text = constants_str::X.repeat(length);
                    let result =
                        crate::storage_path_segment::StoragePathSegment::try_from(text.clone());
                    if (1usize..=1024usize).contains(&length) {
                        result
                            .is_ok_and(|storage_path_segment| storage_path_segment.as_ref() == text)
                    } else {
                        result
                            == Err(
                                crate::storage_path_segment_error::StoragePathSegmentError::Invalid,
                            )
                    }
                })
        );
    }
}
