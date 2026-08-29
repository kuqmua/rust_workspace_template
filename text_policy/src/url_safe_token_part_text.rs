use super::domain_types::{
    URL_SAFE_TOKEN_PART_MAXIMUM_BYTES, UrlSafeTokenPartMaximumBytes, UrlSafeTokenPartRef,
    UrlSafeTokenPartTextError, validate_url_safe_token_part,
};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct UrlSafeTokenPartText(String);
impl TryFrom<String> for UrlSafeTokenPartText {
    type Error = UrlSafeTokenPartTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > URL_SAFE_TOKEN_PART_MAXIMUM_BYTES {
            return Err(Self::Error::TooLong);
        }
        validate_url_safe_token_part(
            UrlSafeTokenPartRef::from(value.as_str()),
            UrlSafeTokenPartMaximumBytes::from(URL_SAFE_TOKEN_PART_MAXIMUM_BYTES),
        )?;
        Ok(Self(value))
    }
}
