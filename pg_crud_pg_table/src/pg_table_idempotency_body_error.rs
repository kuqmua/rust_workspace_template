#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgTableIdempotencyBodyError {
    #[error("{}", constants_str::IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT)]
    TooLarge,
}
