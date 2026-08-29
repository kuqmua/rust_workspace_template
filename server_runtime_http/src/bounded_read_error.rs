#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BoundedReadError {
    #[error("content exceeds maximum size of {maximum_bytes} bytes")]
    ExceedsMaximum {
        maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
    },
    #[error("HTTP response body read failed")]
    Http {
        #[source]
        source: crate::reqwest_error::ReqwestError,
    },
    #[error("file read failed")]
    Io {
        #[source]
        source: crate::bounded_read_io_error::BoundedReadIoError,
    },
    #[error("bounded read concurrency limiter is closed")]
    LimiterClosed,
    #[error("text content must be valid UTF-8")]
    Utf8 {
        #[source]
        source: crate::bounded_read_from_utf8_error::BoundedReadFromUtf8Error,
    },
}
