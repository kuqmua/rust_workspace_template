#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub async fn complete_pg_table_idempotency_in_connection(
    mut connection: SqlxPgTablePgConnectionRef<'_>,
    request: &PgTableIdempotencyRequest,
    response_status: PgTableIdempotencyResponseStatus,
    response_body: PgTableIdempotencyBodyRef<'_>,
) -> Result<(), SqlxPgTableIdempotencyError> {
    if response_body.0.len() > constants_usize::VALUE_1_048_576 {
        return Err(SqlxPgTableIdempotencyError::from(sqlx::Error::Protocol(
            constants_str::IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT.to_owned(),
        )));
    }
    let response_status_i16 = i16::try_from(response_status.0).map_err(|_error| {
        SqlxPgTableIdempotencyError::from(sqlx::Error::Protocol(
            constants_str::IDEMPOTENCY_RESPONSE_STATUS_IS_OUTSIDE_SMALLINT.to_owned(),
        ))
    })?;
    let result = sqlx::query(constants_str::PG_CRUD_COMPLETE_IDEMPOTENCY_SQL)
        .bind(request.scope.actor.0.as_str())
        .bind(request.scope.method.0.as_str())
        .bind(request.scope.route.0.as_str())
        .bind(request.scope.key.0.as_str())
        .bind(request.request_hash.0.as_slice())
        .bind(response_status_i16)
        .bind(response_body.0)
        .execute(connection.as_mut())
        .await
        .map_err(SqlxPgTableIdempotencyError::from)?;
    if result.rows_affected() == 1u64 {
        Ok(())
    } else {
        Err(SqlxPgTableIdempotencyError::from(sqlx::Error::Protocol(
            constants_str::IDEMPOTENCY_RESERVATION_IS_UNAVAILABLE_FOR_COMPLETION.to_owned(),
        )))
    }
}
