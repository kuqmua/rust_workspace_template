#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BoundedReadError {
    #[error("content exceeds maximum size of {maximum_bytes} bytes")]
    ExceedsMaximum {
        maximum_bytes: super::BoundedReadMaximumBytes,
    },
    #[error("HTTP response body read failed")]
    Http {
        #[source]
        source: super::ReqwestError,
    },
    #[error("file read failed")]
    Io {
        #[source]
        source: super::BoundedReadIoError,
    },
    #[error("bounded read concurrency limiter is closed")]
    LimiterClosed,
    #[error("text content must be valid UTF-8")]
    Utf8 {
        #[source]
        source: super::BoundedReadFromUtf8Error,
    },
}
