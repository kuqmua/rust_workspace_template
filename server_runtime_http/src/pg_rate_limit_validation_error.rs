#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PgRateLimitValidationError {
    #[error("rate-limit key part must not be empty")]
    EmptyKeyPart,
    #[error("rate-limit key part is too long")]
    KeyPartTooLong,
    #[error("rate-limit numeric configuration must be positive")]
    MustBePositive,
}
