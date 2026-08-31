pub async fn begin_pg_table_idempotency(
    pool: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    request: &crate::pg_table_idempotency_request::PgTableIdempotencyRequest,
) -> Result<
    crate::pg_table_idempotency_begin::PgTableIdempotencyBegin,
    crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError,
> {
    let inserted = sqlx::query_scalar::<_, bool>(constants_str::INSERT_INTO_PG_TABLE_IDEMPOTENCY_ACTOR_HTTP_METHOD_ROUTE_PATH_IDEMPOTENCY_KEY)
        .bind(request.scope.actor.0.as_str())
        .bind(request.scope.method.0.as_str())
        .bind(request.scope.route.0.as_str())
        .bind(request.scope.key.as_ref())
        .bind(request.request_hash.0.as_slice())
        .fetch_optional(pool.as_ref())
        .await
        .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    if inserted == Some(true) {
        return Ok(crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired);
    }
    let existing = sqlx::query_as::<_, (Vec<u8>, String, Option<i16>, Option<Vec<u8>>)>(
        constants_str::SELECT_REQUEST_HASH_STATE_RESPONSE_STATUS_RESPONSE_BODY_FROM_PG_TABLE_IDEMPOTENCY,
    )
    .bind(request.scope.actor.0.as_str())
    .bind(request.scope.method.0.as_str())
    .bind(request.scope.route.0.as_str())
    .bind(request.scope.key.as_ref())
    .fetch_one(pool.as_ref())
    .await
    .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    if existing.0.as_slice() != request.request_hash.0.as_slice() {
        return Ok(crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::Conflict);
    }
    if existing.1 == constants_str::PENDING {
        return Ok(crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress);
    }
    match (existing.2, existing.3) {
        (Some(status), Some(raw_response_body)) => {
            let response_status = match u16::try_from(status) {
                Ok(value) => value,
                Err(_error) => {
                    return Ok(
                        crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress,
                    );
                }
            };
            let response_body =
                match crate::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(
                    raw_response_body,
                ) {
                    Ok(value) => value,
                    Err(_error) => {
                        return Ok(
                            crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress,
                        );
                    }
                };
            Ok(crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::Replay(crate::pg_table_idempotency_replay::PgTableIdempotencyReplay {
                response_body,
                response_status: match crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(response_status) {
                    Ok(value) => value,
                    Err(_error) => return Ok(crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress),
                },
            }))
        }
        (None | Some(_), None) | (None, Some(_)) => {
            Ok(crate::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress)
        }
    }
}
