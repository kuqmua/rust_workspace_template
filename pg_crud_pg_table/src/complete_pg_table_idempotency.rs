#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub async fn complete_pg_table_idempotency(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    pg_table_idempotency_request: &crate::pg_table_idempotency_request::PgTableIdempotencyRequest,
    pg_table_idempotency_response_status: crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus,
    pg_table_idempotency_body_ref: crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<
        '_,
    >,
) -> Result<(), crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError> {
    if pg_table_idempotency_body_ref.as_ref().len() > constants_usize::VALUE_1_048_576 {
        return crate::release_pg_table_idempotency::release_pg_table_idempotency(
            sqlx_pg_pool_ref,
            pg_table_idempotency_request,
        )
        .await;
    }
    let response_status_i16 = match i16::try_from(u16::from(pg_table_idempotency_response_status)) {
        Ok(value) => value,
        Err(_error) => {
            return crate::release_pg_table_idempotency::release_pg_table_idempotency(
                sqlx_pg_pool_ref,
                pg_table_idempotency_request,
            )
            .await;
        }
    };
    let _query_result = sqlx::query(constants_str::PG_CRUD_COMPLETE_IDEMPOTENCY_SQL)
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
        .bind(response_status_i16)
        .bind(pg_table_idempotency_body_ref.as_ref())
        .execute(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
