#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum StdRequestTimeoutTryFromDurationError {
    #[error("{}", constants_str::REQUEST_TIMEOUT_MUST_BE_GREATER_THAN_ZERO)]
    Zero,
}
