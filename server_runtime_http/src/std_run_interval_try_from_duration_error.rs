#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum StdRunIntervalTryFromDurationError {
    #[error("{}", constants_str::RUN_INTERVAL_MUST_BE_GREATER_THAN_ZERO)]
    Zero,
}
