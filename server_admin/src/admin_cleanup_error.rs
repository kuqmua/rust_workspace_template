#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminCleanupError {
    #[error("{}", constants_str::ADMIN_CLEANUP_ROWS_EXCEED_I64)]
    Count,
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_IDEMPOTENCY_CLEANUP_FAILED)]
    Idempotency(#[source] pg_table::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError),
    #[error(transparent)]
    IdempotencyConfig(#[from] pg_table::pg_table_idempotency_cleanup_value_try_from_i64_error::PgTableIdempotencyCleanupValueTryFromI64Error),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_TABLE_CLEANUP_FAILED)]
    Pg(#[source] crate::sqlx_admin_error::SqlxAdminError),
}
