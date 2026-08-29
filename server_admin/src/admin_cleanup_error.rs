#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminCleanupError {
    #[error("{}", constants_str::catalog::ADMIN_CLEANUP_ROWS_EXCEED_I64)]
    Count,
    #[error("idempotency cleanup failed: {0}")]
    Idempotency(#[source] pg_table::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError),
    #[error(transparent)]
    IdempotencyConfig(#[from] pg_table::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error),
    #[error("administrator table cleanup failed: {0:?}")]
    Pg(#[source] crate::sqlx_admin_error::SqlxAdminError),
}
