#[must_use]
pub fn resolve_header_text<'header>(
    headers: crate::http_header_map_ref::HttpHeaderMapRef<'header>,
    name: &crate::http_header_name::HttpHeaderName,
    maximum: crate::http_header_text_maximum_bytes::HttpHeaderTextMaximumBytes,
) -> crate::http_header_text_resolution::HttpHeaderTextResolution<'header> {
    let Some(value) = headers.get().get(name.as_ref()) else {
        return crate::http_header_text_resolution::HttpHeaderTextResolution::Missing;
    };
    let bytes = value.as_bytes();
    if bytes.len() > usize::from(maximum) {
        return crate::http_header_text_resolution::HttpHeaderTextResolution::ExceedsMaximumBytes {
            actual_bytes: crate::http_header_text_bytes::HttpHeaderTextBytes::from(bytes.len()),
        };
    }
    match value.to_str() {
        Ok(text) => crate::http_header_text_resolution::HttpHeaderTextResolution::Value(
            crate::http_header_text_ref::HttpHeaderTextRef::from(text.trim()),
        ),
        Err(_error) => crate::http_header_text_resolution::HttpHeaderTextResolution::InvalidText,
    }
}
