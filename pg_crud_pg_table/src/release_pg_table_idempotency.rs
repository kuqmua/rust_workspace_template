#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub async fn release_pg_table_idempotency(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    request: &PgTableIdempotencyRequest,
) -> Result<(), SqlxPgTableIdempotencyError> {
    let _query_result = sqlx::query(
        constants_str::DELETE_FROM_PG_TABLE_IDEMPOTENCY_WHERE_ACTOR_DOLLAR_1_AND_HTTP_METHOD,
    )
    .bind(request.scope.actor.0.as_str())
    .bind(request.scope.method.0.as_str())
    .bind(request.scope.route.0.as_str())
    .bind(request.scope.key.0.as_str())
    .bind(request.request_hash.0.as_slice())
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
