#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::catalog::RUN_INTERVAL_MUST_BE_GREATER_THAN_ZERO)]
pub struct StdRunIntervalTryFromDurationError;
