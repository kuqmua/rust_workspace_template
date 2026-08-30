#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum StdLeaseStaleTimeoutError {
    #[error("lease stale timeout must be greater than zero")]
    Zero,
}
