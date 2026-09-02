#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum StaleStagingCleanupCfgError {
    #[error("stale staging cleanup limit must be between 1 and 10000")]
    InvalidLimit,
}
