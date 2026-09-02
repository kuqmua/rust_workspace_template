#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum RetryAfterSecsTryFromU64Error {
    #[error("{}", constants_str::RETRY_AFTER_SECONDS_MUST_BE_GREATER_THAN_ZERO)]
    Zero,
}
