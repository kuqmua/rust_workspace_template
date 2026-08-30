#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum CleanupBatchSizeError {
    #[error("cleanup batch size must be greater than zero")]
    Zero,
}
