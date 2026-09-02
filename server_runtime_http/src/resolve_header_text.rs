#[must_use]
pub fn resolve_header_text<'header>(
    http_header_map_ref: crate::http_header_map_ref::HttpHeaderMapRef<'header>,
    http_header_name: &crate::http_header_name::HttpHeaderName,
    http_header_text_maximum_bytes: crate::http_header_text_maximum_bytes::HttpHeaderTextMaximumBytes,
) -> crate::http_header_text_resolution::HttpHeaderTextResolution<'header> {
    let Some(value) = http_header_map_ref.get().get(http_header_name.as_ref()) else {
        return crate::http_header_text_resolution::HttpHeaderTextResolution::Missing;
    };
    let bytes = value.as_bytes();
    if bytes.len() > usize::from(http_header_text_maximum_bytes) {
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
