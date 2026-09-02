pub fn validate_url_safe_token_part(
    url_safe_token_part_ref: crate::url_safe_token_part_ref::UrlSafeTokenPartRef<'_>,
    url_safe_token_part_maximum_bytes: crate::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes,
) -> Result<(), crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError> {
    let value_text: &str = url_safe_token_part_ref.into();
    if value_text.len() > usize::from(url_safe_token_part_maximum_bytes) {
        return Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::TooLong);
    }
    if value_text.is_empty() {
        Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::Empty)
    } else if value_text
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        Ok(())
    } else {
        Err(crate::url_safe_token_part_text_error::UrlSafeTokenPartTextError::InvalidSymbol)
    }
}
