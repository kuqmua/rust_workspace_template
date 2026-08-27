#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpContentLengthError {
    #[error("Content-Length must not be empty")]
    Empty,
    #[error("Content-Length must contain only ASCII digits")]
    InvalidSymbol,
    #[error("Content-Length exceeds u64")]
    OutOfRange,
    #[error("Content-Length contains too many digits")]
    TooLong,
}
