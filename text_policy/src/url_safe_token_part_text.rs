#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct UrlSafeTokenPartText(String);
impl TryFrom<String> for UrlSafeTokenPartText {
    type Error = crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::url_safe_token_part_maximum_bytes::URL_SAFE_TOKEN_PART_MAXIMUM_BYTES
        {
            return Err(Self::Error::TooLong);
        }
        crate::validate_url_safe_token_part::validate_url_safe_token_part(
            crate::url_safe_token_part_ref::UrlSafeTokenPartRef::from(value.as_str()),
            crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                crate::url_safe_token_part_maximum_bytes::URL_SAFE_TOKEN_PART_MAXIMUM_BYTES,
            ),
        )?;
        Ok(Self(value))
    }
}
