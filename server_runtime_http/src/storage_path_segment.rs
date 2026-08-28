#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct StoragePathSegment(String);

impl TryFrom<String> for StoragePathSegment {
    type Error = super::StoragePathSegmentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_1_024 {
            return Err(super::StoragePathSegmentError);
        }
        text_policy::domain_types::validate_url_safe_token_part(
            text_policy::domain_types::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::domain_types::UrlSafeTokenPartMaximumBytes::from(
                constants_usize::VALUE_1_024,
            ),
        )
        .map_err(|_error| super::StoragePathSegmentError)?;
        Ok(Self(value))
    }
}
