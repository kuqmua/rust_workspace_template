#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpHeaderTextMaximumBytesError {
    #[error("HTTP header text maximum must be greater than zero")]
    Zero,
}
