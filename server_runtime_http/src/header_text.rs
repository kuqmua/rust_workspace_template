#[cfg(test)]
mod tests {
    #[test]
    fn resolution_distinguishes_missing_invalid_oversized_and_valid_values() {
        let maximum = crate::http_header_text_maximum_bytes::HttpHeaderTextMaximumBytes::try_from(5usize).expect("84792c6a resolution_distinguishes_missing_invalid_oversized_and_valid_values invariant must hold");
        let name = crate::http_header_name::HttpHeaderName::from(
            http::header::HeaderName::from_static(constants_str::catalog::TEST_X_TEST_HEADER),
        );
        let mut headers = http::HeaderMap::new();
        assert_eq!(
            crate::resolve_header_text::resolve_header_text(
                crate::http_header_map_ref::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            crate::http_header_text_resolution::HttpHeaderTextResolution::Missing
        );
        let _invalid_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_bytes(&[0xffu8]).expect("fd47f469 resolution_distinguishes_missing_invalid_oversized_and_valid_values invariant must hold"),
        );
        assert_eq!(
            crate::resolve_header_text::resolve_header_text(
                crate::http_header_map_ref::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            crate::http_header_text_resolution::HttpHeaderTextResolution::InvalidText
        );
        let _oversized_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_static(constants_str::catalog::VALUE_123456),
        );
        assert_eq!(
            crate::resolve_header_text::resolve_header_text(
                crate::http_header_map_ref::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            crate::http_header_text_resolution::HttpHeaderTextResolution::ExceedsMaximumBytes {
                actual_bytes: crate::http_header_text_bytes::HttpHeaderTextBytes::from(6usize)
            }
        );
        let _valid_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_static(constants_str::catalog::TEST_TRIMMED_OK),
        );
        assert_eq!(
            crate::resolve_header_text::resolve_header_text(
                crate::http_header_map_ref::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            crate::http_header_text_resolution::HttpHeaderTextResolution::Value(
                crate::http_header_text_ref::HttpHeaderTextRef(constants_str::catalog::OK_ALT)
            )
        );
    }

    #[test]
    fn maximum_rejects_zero() {
        assert_eq!(
            crate::http_header_text_maximum_bytes::HttpHeaderTextMaximumBytes::try_from(
                constants_usize::ZERO
            ),
            Err(crate::http_header_text_maximum_bytes_error::HttpHeaderTextMaximumBytesError)
        );
    }
}

// Root-owned module compatibility wrappers.
mod http_header_name {}
mod http_header_text_bytes {}
mod http_header_text_maximum_bytes {}
mod http_header_text_maximum_bytes_error {}
mod http_header_text_ref {}
mod http_header_text_resolution {}
