#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
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
