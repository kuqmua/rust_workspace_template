#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub async fn ensure_pg_table_idempotency_schema(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<(), crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError> {
    let mut transaction = sqlx_pg_pool_ref
        .as_ref()
        .begin()
        .await
        .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    let _schema_lock = sqlx::query(constants_str::PG_TABLE_IDEMPOTENCY_SCHEMA_LOCK_SQL)
        .execute(&mut *transaction)
        .await
        .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    let _query_result = sqlx::query(
        constants_str::CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ACTOR_TEXT_NOT_NULL,
    )
    .execute(&mut *transaction)
    .await
    .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    let _index_result = sqlx::query(
        constants_str::CREATE_INDEX_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_CREATED_AT_IDX_ON,
    )
    .execute(&mut *transaction)
    .await
    .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    transaction
        .commit()
        .await
        .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)
}
