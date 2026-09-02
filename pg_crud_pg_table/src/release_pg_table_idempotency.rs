#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub async fn release_pg_table_idempotency(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    pg_table_idempotency_request: &crate::pg_table_idempotency_request::PgTableIdempotencyRequest,
) -> Result<(), crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError> {
    let _query_result = sqlx::query(
        constants_str::DELETE_FROM_PG_TABLE_IDEMPOTENCY_WHERE_ACTOR_DOLLAR_1_AND_HTTP_METHOD,
    )
    .bind(
        pg_table_idempotency_request
            .get_scope()
            .get_actor()
            .as_ref(),
    )
    .bind(
        pg_table_idempotency_request
            .get_scope()
            .get_method()
            .as_ref(),
    )
    .bind(
        pg_table_idempotency_request
            .get_scope()
            .get_route()
            .as_ref(),
    )
    .bind(pg_table_idempotency_request.get_scope().get_key().as_ref())
    .bind(
        pg_table_idempotency_request
            .get_request_hash()
            .get()
            .as_slice(),
    )
    .execute(sqlx_pg_pool_ref.as_ref())
    .await
    .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
