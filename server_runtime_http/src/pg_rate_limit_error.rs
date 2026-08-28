#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum PgRateLimitError {
    #[error("PostgreSQL rate-limit query failed: {0}")]
    Sqlx(super::SqlxPgRateLimitError),
}
