#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("cleanup batch size must be greater than zero")]
pub struct CleanupBatchSizeError;
