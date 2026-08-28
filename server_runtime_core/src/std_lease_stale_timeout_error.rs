#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("lease stale timeout must be greater than zero")]
pub struct StdLeaseStaleTimeoutError;
