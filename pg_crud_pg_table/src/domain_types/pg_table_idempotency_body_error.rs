#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT)]
pub struct PgTableIdempotencyBodyError;
