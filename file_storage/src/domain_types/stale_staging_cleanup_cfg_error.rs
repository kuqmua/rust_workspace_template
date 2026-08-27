#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("stale staging cleanup limit must be between 1 and 10000")]
pub struct StaleStagingCleanupCfgError;
