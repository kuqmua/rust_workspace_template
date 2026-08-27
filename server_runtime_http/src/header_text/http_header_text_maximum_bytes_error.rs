#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("HTTP header text maximum must be greater than zero")]
pub struct HttpHeaderTextMaximumBytesError;
