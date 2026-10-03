#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum HttpCorsAllowOriginHeaderValuesError {
    #[error("CORS allow-origin configuration contains an invalid origin")]
    InvalidOrigin,
    #[error("CORS allow-origin configuration exceeds its maximum byte length")]
    TooLong,
    #[error("CORS allow-origin configuration contains too many entries")]
    TooManyItems,
}

impl From<crate::allowed_origin_error::AllowedOriginError>
    for HttpCorsAllowOriginHeaderValuesError
{
    fn from(value: crate::allowed_origin_error::AllowedOriginError) -> Self {
        let _: crate::allowed_origin_error::AllowedOriginError = value;
        Self::InvalidOrigin
    }
}

impl From<http::header::InvalidHeaderValue> for HttpCorsAllowOriginHeaderValuesError {
    fn from(value: http::header::InvalidHeaderValue) -> Self {
        let _: http::header::InvalidHeaderValue = value;
        Self::InvalidOrigin
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cors_origin_errors_classify_invalid_origin_and_header_value() {
        assert_eq!(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::from(crate::allowed_origin_error::AllowedOriginError::Invalid), crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::InvalidOrigin);
        let header_result = http::HeaderValue::from_bytes(&[0u8]);
        assert!(header_result.is_err());
        let Err(invalid_header_value) = header_result else {
            return;
        };
        assert_eq!(crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::from(invalid_header_value), crate::http_cors_allow_origin_header_values_error::HttpCorsAllowOriginHeaderValuesError::InvalidOrigin);
    }
}
