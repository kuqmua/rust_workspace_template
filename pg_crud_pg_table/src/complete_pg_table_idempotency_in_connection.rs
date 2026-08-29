#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub async fn complete_pg_table_idempotency_in_connection(
    mut connection: crate::sqlx_pg_table_pg_connection_ref::SqlxPgTablePgConnectionRef<'_>,
    request: &crate::pg_table_idempotency_request::PgTableIdempotencyRequest,
    response_status: crate::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus,
    response_body: crate::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<'_>,
) -> Result<(), crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError> {
    if response_body.0.len() > constants_usize::VALUE_1_048_576 {
        return Err(
            crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from(
                sqlx::Error::Protocol(
                    constants_str::catalog::IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT
                        .to_owned(),
                ),
            ),
        );
    }
    let response_status_i16 = i16::try_from(response_status.0).map_err(|_error| {
        crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from(
            sqlx::Error::Protocol(
                constants_str::catalog::IDEMPOTENCY_RESPONSE_STATUS_IS_OUTSIDE_SMALLINT.to_owned(),
            ),
        )
    })?;
    let result = sqlx::query(constants_str::catalog::PG_CRUD_COMPLETE_IDEMPOTENCY_SQL)
        .bind(request.scope.actor.0.as_str())
        .bind(request.scope.method.0.as_str())
        .bind(request.scope.route.0.as_str())
        .bind(request.scope.key.0.as_str())
        .bind(request.request_hash.0.as_slice())
        .bind(response_status_i16)
        .bind(response_body.0)
        .execute(connection.as_mut())
        .await
        .map_err(crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from)?;
    if result.rows_affected() == 1u64 {
        Ok(())
    } else {
        Err(
            crate::sqlx_pg_table_idempotency_error::SqlxPgTableIdempotencyError::from(
                sqlx::Error::Protocol(
                    constants_str::catalog::IDEMPOTENCY_RESERVATION_IS_UNAVAILABLE_FOR_COMPLETION
                        .to_owned(),
                ),
            ),
        )
    }
}
