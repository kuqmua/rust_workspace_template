#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub async fn cleanup_pg_table_idempotency(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    completed_retention_seconds: PgTableIdempotencyCleanupRetentionSeconds,
    pending_retention_seconds: PgTableIdempotencyCleanupRetentionSeconds,
    batch_size: PgTableIdempotencyCleanupBatchSize,
) -> Result<PgTableIdempotencyCleanupRows, SqlxPgTableIdempotencyError> {
    let result = sqlx::query(
        constants_str::WITH_EXPIRED_AS_SELECT_ACTOR_HTTP_METHOD_ROUTE_PATH_IDEMPOTENCY_KEY_FROM,
    )
    .bind(completed_retention_seconds.0)
    .bind(pending_retention_seconds.0)
    .bind(batch_size.0.0.get())
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(PgTableIdempotencyCleanupRows::from(result.rows_affected()))
}
