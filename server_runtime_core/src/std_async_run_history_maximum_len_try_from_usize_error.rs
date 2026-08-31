#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum StdAsyncRunHistoryMaximumLenTryFromUsizeError {
    #[error(
        "{}",
        constants_str::RUN_HISTORY_MAXIMUM_LENGTH_MUST_BE_GREATER_THAN_ZERO
    )]
    Zero,
}
