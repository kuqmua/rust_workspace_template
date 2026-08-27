#[must_use]
pub fn resolve_header_text<'header>(
    headers: super::HttpHeaderMapRef<'header>,
    name: &crate::domain_types::HttpHeaderName,
    maximum: crate::domain_types::HttpHeaderTextMaximumBytes,
) -> crate::domain_types::HttpHeaderTextResolution<'header> {
    let Some(value) = headers.0.get(name.as_ref()) else {
        return crate::domain_types::HttpHeaderTextResolution::Missing;
    };
    let bytes = value.as_bytes();
    if bytes.len() > usize::from(maximum) {
        return crate::domain_types::HttpHeaderTextResolution::ExceedsMaximumBytes {
            actual_bytes: crate::domain_types::HttpHeaderTextBytes::from(bytes.len()),
        };
    }
    match value.to_str() {
        Ok(text) => crate::domain_types::HttpHeaderTextResolution::Value(
            crate::domain_types::HttpHeaderTextRef::from(text.trim()),
        ),
        Err(_error) => crate::domain_types::HttpHeaderTextResolution::InvalidText,
    }
}
