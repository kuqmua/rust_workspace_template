use super::{UrlSafeTokenPartMaximumBytes, UrlSafeTokenPartRef, UrlSafeTokenPartTextError};

pub fn validate_url_safe_token_part(
    value: UrlSafeTokenPartRef<'_>,
    maximum_bytes: UrlSafeTokenPartMaximumBytes,
) -> Result<(), UrlSafeTokenPartTextError> {
    if value.0.len() > maximum_bytes.0 {
        return Err(UrlSafeTokenPartTextError::TooLong);
    }
    if value.0.is_empty() {
        Err(UrlSafeTokenPartTextError::Empty)
    } else if value
        .0
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        Ok(())
    } else {
        Err(UrlSafeTokenPartTextError::InvalidSymbol)
    }
}
