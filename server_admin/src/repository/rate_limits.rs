#![allow(clippy::single_call_fn)] // the typed function owns the PostgreSQL rate-limit contract

pub(crate) async fn enforce_rate_limit(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    scope: crate::StdAdminStrRef<'_>,
    subject: &crate::StdAdminString,
    limit: crate::auth::StdAdminRateLimitCount,
    window_seconds: crate::auth::StdAdminRateLimitWindowSeconds,
) -> Result<super::AdminRateLimitOutcome, super::AdminRateLimitRepositoryError> {
    server_runtime::enforce_pg_rate_limit(
        server_runtime::SqlxPgRateLimitPoolRef::from(pool.0),
        server_runtime::PgRateLimitQueryRef::from(
            str_constants::SERVER_ADMIN_ENFORCE_RATE_LIMIT_SQL,
        ),
        server_runtime::PgRateLimitScopeRef::try_from(scope.as_ref())
            .map_err(|_error| super::AdminRateLimitRepositoryError::InvalidPolicy)?,
        server_runtime::PgRateLimitSubjectRef::try_from(subject.as_ref().as_str())
            .map_err(|_error| super::AdminRateLimitRepositoryError::InvalidPolicy)?,
        server_runtime::PgRateLimitMaximum::try_from(i64::from(limit))
            .map_err(|_error| super::AdminRateLimitRepositoryError::InvalidPolicy)?,
        server_runtime::PgRateLimitWindowSeconds::try_from(i32::from(window_seconds))
            .map_err(|_error| super::AdminRateLimitRepositoryError::InvalidPolicy)?,
    )
    .await
    .map_err(|error| match error {
        server_runtime::PgRateLimitError::Sqlx(source) => {
            super::AdminRateLimitRepositoryError::Sqlx(crate::SqlxAdminError::from(
                sqlx::Error::from(source),
            ))
        }
    })
    .map(|decision| match decision {
        server_runtime::PgRateLimitDecision::Allowed => super::AdminRateLimitOutcome::Allowed,
        server_runtime::PgRateLimitDecision::Limited(_retry_after) => {
            super::AdminRateLimitOutcome::Limited
        }
    })
}
