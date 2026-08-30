#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct StoragePathSegment(String);

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
        Ok(Self(value))
    }
}
