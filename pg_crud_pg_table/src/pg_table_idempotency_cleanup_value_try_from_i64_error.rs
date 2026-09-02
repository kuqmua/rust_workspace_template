#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PgTableIdempotencyCleanupValueTryFromI64Error {
    #[error("{self:?}")]
    Negative,
    #[error("{self:?}")]
    NotPositive,
}
