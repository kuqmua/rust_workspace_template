#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("content security policy exceeds 4096 bytes")]
pub struct HttpCspMaximumBytesError;
