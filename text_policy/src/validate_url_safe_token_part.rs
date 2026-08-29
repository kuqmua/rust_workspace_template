pub fn validate_url_safe_token_part(
    value: crate::url_safe_token_part_ref::UrlSafeTokenPartRef<'_>,
    maximum_bytes: crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes,
) -> Result<(), crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError> {
    if value.0.len() > maximum_bytes.0 {
        return Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong);
    }
    if value.0.is_empty() {
        Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::Empty)
    } else if value
        .0
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        Ok(())
    } else {
        Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::InvalidSymbol)
    }
}
