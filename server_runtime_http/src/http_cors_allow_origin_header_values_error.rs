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
    fn from(allowed_origin_error: crate::allowed_origin_error::AllowedOriginError) -> Self {
        let _: crate::allowed_origin_error::AllowedOriginError = allowed_origin_error;
        Self::InvalidOrigin
    }
}

impl From<http::header::InvalidHeaderValue> for HttpCorsAllowOriginHeaderValuesError {
    fn from(invalid_header_value: http::header::InvalidHeaderValue) -> Self {
        let _: http::header::InvalidHeaderValue = invalid_header_value;
        Self::InvalidOrigin
    }
}
