pub use crate::http_header_name::HttpHeaderName;
pub use crate::http_header_text_bytes::HttpHeaderTextBytes;
pub use crate::http_header_text_maximum_bytes::HttpHeaderTextMaximumBytes;
pub use crate::http_header_text_maximum_bytes_error::HttpHeaderTextMaximumBytesError;
pub use crate::http_header_text_ref::HttpHeaderTextRef;
pub use crate::http_header_text_resolution::HttpHeaderTextResolution;

#[cfg(test)]
mod tests {
    #[test]
    fn resolution_distinguishes_missing_invalid_oversized_and_valid_values() {
        let maximum = super::HttpHeaderTextMaximumBytes::try_from(5usize).expect("84792c6a resolution_distinguishes_missing_invalid_oversized_and_valid_values invariant must hold");
        let name = super::HttpHeaderName::from(http::header::HeaderName::from_static(
            constants_str::TEST_X_TEST_HEADER,
        ));
        let mut headers = http::HeaderMap::new();
        assert_eq!(
            crate::domain_types::resolve_header_text(
                crate::domain_types::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            super::HttpHeaderTextResolution::Missing
        );
        let _invalid_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_bytes(&[0xffu8]).expect("fd47f469 resolution_distinguishes_missing_invalid_oversized_and_valid_values invariant must hold"),
        );
        assert_eq!(
            crate::domain_types::resolve_header_text(
                crate::domain_types::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            super::HttpHeaderTextResolution::InvalidText
        );
        let _oversized_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_static(constants_str::VALUE_123456),
        );
        assert_eq!(
            crate::domain_types::resolve_header_text(
                crate::domain_types::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            super::HttpHeaderTextResolution::ExceedsMaximumBytes {
                actual_bytes: super::HttpHeaderTextBytes::from(6usize)
            }
        );
        let _valid_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_static(constants_str::TEST_TRIMMED_OK),
        );
        assert_eq!(
            crate::domain_types::resolve_header_text(
                crate::domain_types::HttpHeaderMapRef::from(&headers),
                &name,
                maximum
            ),
            super::HttpHeaderTextResolution::Value(super::HttpHeaderTextRef(constants_str::OK_ALT))
        );
    }

    #[test]
    fn maximum_rejects_zero() {
        assert_eq!(
            super::HttpHeaderTextMaximumBytes::try_from(constants_usize::ZERO),
            Err(super::HttpHeaderTextMaximumBytesError)
        );
    }
}

// Root-owned module compatibility wrappers.
mod http_header_name {
    pub use crate::http_header_name::*;
}
mod http_header_text_bytes {
    pub use crate::http_header_text_bytes::*;
}
mod http_header_text_maximum_bytes {
    pub use crate::http_header_text_maximum_bytes::*;
}
mod http_header_text_maximum_bytes_error {
    pub use crate::http_header_text_maximum_bytes_error::*;
}
mod http_header_text_ref {
    pub use crate::http_header_text_ref::*;
}
mod http_header_text_resolution {
    pub use crate::http_header_text_resolution::*;
}
