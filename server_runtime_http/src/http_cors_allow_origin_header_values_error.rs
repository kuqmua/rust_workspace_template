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

impl From<super::super::AllowedOriginError> for HttpCorsAllowOriginHeaderValuesError {
    fn from(_value: super::super::AllowedOriginError) -> Self {
        Self::InvalidOrigin
    }
}

impl From<http::header::InvalidHeaderValue> for HttpCorsAllowOriginHeaderValuesError {
    fn from(_value: http::header::InvalidHeaderValue) -> Self {
        Self::InvalidOrigin
    }
}
