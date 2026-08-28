#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminCleanupError {
    #[error("{}", constants_str::ADMIN_CLEANUP_ROWS_EXCEED_I64)]
    Count,
    #[error("idempotency cleanup failed: {0}")]
    Idempotency(#[source] pg_table::SqlxPgTableIdempotencyError),
    #[error(transparent)]
    IdempotencyConfig(#[from] pg_table::PgTableIdempotencyCleanupValueTryFromI64Error),
    #[error("administrator table cleanup failed: {0:?}")]
    Pg(#[source] crate::SqlxAdminError),
}
