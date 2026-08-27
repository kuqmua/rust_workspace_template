#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("retry attempts must be greater than zero")]
pub struct StdRetryAttemptsError;
