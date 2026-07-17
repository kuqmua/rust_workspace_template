#![allow(clippy::single_call_fn)] // the typed function owns the PostgreSQL rate-limit contract

const ENFORCE_RATE_LIMIT: &str = "INSERT INTO admin_rate_limits (scope, subject, window_started_at, request_count) VALUES ($1, $2, now(), 1) ON CONFLICT (scope, subject) DO UPDATE SET window_started_at = CASE WHEN admin_rate_limits.window_started_at <= now() - make_interval(secs => $4) THEN now() ELSE admin_rate_limits.window_started_at END, request_count = CASE WHEN admin_rate_limits.window_started_at <= now() - make_interval(secs => $4) THEN 1 ELSE admin_rate_limits.request_count + 1 END RETURNING request_count <= $3";

pub(crate) async fn enforce_rate_limit(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    scope: crate::StdAdminStrRef<'_>,
    subject: &crate::StdAdminString,
    limit: crate::auth::StdAdminRateLimitCount,
    window_seconds: crate::auth::StdAdminRateLimitWindowSeconds,
) -> Result<super::AdminRateLimitOutcome, super::AdminRateLimitRepositoryError> {
    server_runtime::enforce_pg_rate_limit(
        server_runtime::SqlxPgRateLimitPoolRef::from(pool.0),
        server_runtime::PgRateLimitQueryRef::from(ENFORCE_RATE_LIMIT),
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
