#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub async fn ensure_pg_table_idempotency_schema(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), SqlxPgTableIdempotencyError> {
    let _query_result = sqlx::query(
        constants_str::CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ACTOR_TEXT_NOT_NULL,
    )
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    let _index_result = sqlx::query(
        constants_str::CREATE_INDEX_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_CREATED_AT_IDX_ON,
    )
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
