#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
)]
pub struct HttpHeaderTextMaximumBytes(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HttpHeaderTextBytes(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct HttpHeaderName(http::HeaderName);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct HttpHeaderTextRef<'header>(&'header str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("HTTP header text maximum must be greater than zero")]
pub struct HttpHeaderTextMaximumBytesError;

impl TryFrom<usize> for HttpHeaderTextMaximumBytes {
    type Error = HttpHeaderTextMaximumBytesError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == constants_usize::ZERO {
            return Err(HttpHeaderTextMaximumBytesError);
        }
        Ok(Self(value))
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpHeaderTextResolution<'header> {
    ExceedsMaximumBytes { actual_bytes: HttpHeaderTextBytes },
    InvalidText,
    Missing,
    Value(HttpHeaderTextRef<'header>),
}

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
            crate::resolve_header_text(crate::HttpHeaderMapRef::from(&headers), &name, maximum),
            super::HttpHeaderTextResolution::Missing
        );
        let _invalid_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_bytes(&[0xffu8]).expect("fd47f469 resolution_distinguishes_missing_invalid_oversized_and_valid_values invariant must hold"),
        );
        assert_eq!(
            crate::resolve_header_text(crate::HttpHeaderMapRef::from(&headers), &name, maximum),
            super::HttpHeaderTextResolution::InvalidText
        );
        let _oversized_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_static(constants_str::VALUE_123456),
        );
        assert_eq!(
            crate::resolve_header_text(crate::HttpHeaderMapRef::from(&headers), &name, maximum),
            super::HttpHeaderTextResolution::ExceedsMaximumBytes {
                actual_bytes: super::HttpHeaderTextBytes::from(6usize)
            }
        );
        let _valid_previous = headers.insert(
            name.as_ref(),
            http::HeaderValue::from_static(constants_str::TEST_TRIMMED_OK),
        );
        assert_eq!(
            crate::resolve_header_text(crate::HttpHeaderMapRef::from(&headers), &name, maximum),
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
