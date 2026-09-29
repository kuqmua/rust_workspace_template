pub fn validate_https_url_text(
    https_url_text_ref: crate::https_url_text_ref::HttpsUrlTextRef<'_>,
) -> Result<(), crate::https_url_text_error::HttpsUrlTextError> {
    let value: &str = https_url_text_ref.into();
    let remainder = value
        .strip_prefix(constants_str::HTTPS_SCHEME_PREFIX)
        .ok_or(crate::https_url_text_error::HttpsUrlTextError::Invalid)?;
    if value.chars().any(char::is_whitespace)
        || value
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte == b'\\')
    {
        return Err(crate::https_url_text_error::HttpsUrlTextError::Invalid);
    }
    let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default();
    if authority.contains('@') {
        return Err(crate::https_url_text_error::HttpsUrlTextError::Invalid);
    }
    let (host, port) = authority
        .split_once(':')
        .map_or((authority, None), |(host, port)| (host, Some(port)));
    let host_valid = host.contains('.')
        && host.split('.').all(|label| {
            label.chars().next().is_some_and(char::is_alphanumeric)
                && label.chars().last().is_some_and(char::is_alphanumeric)
                && label
                    .chars()
                    .all(|character| character.is_alphanumeric() || character == '-')
        });
    let port_valid = port.is_none_or(|port_text| {
        !port_text.is_empty()
            && port_text.bytes().all(|byte| byte.is_ascii_digit())
            && port_text.parse::<u16>().is_ok_and(|number| number > 0)
    });
    if host_valid && port_valid {
        Ok(())
    } else {
        Err(crate::https_url_text_error::HttpsUrlTextError::Invalid)
    }
}
