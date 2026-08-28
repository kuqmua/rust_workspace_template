#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpContentDispositionError {
    #[error("attachment file name must not be empty")]
    Empty,
    #[error("generated Content-Disposition header value is invalid")]
    InvalidHeaderValue,
    #[error("attachment file name is too long")]
    TooLong,
}
