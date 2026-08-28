pub async fn enforce_pg_rate_limit(
    pool: super::SqlxPgRateLimitPoolRef<'_>,
    query: super::PgRateLimitQueryRef,
    scope: super::PgRateLimitScopeRef<'_>,
    subject: super::PgRateLimitSubjectRef<'_>,
    maximum: super::PgRateLimitMaximum,
    window_seconds: super::PgRateLimitWindowSeconds,
) -> Result<super::PgRateLimitDecision, super::PgRateLimitError> {
    sqlx::query_scalar::<_, bool>(query.0)
        .bind(scope.0)
        .bind(subject.0)
        .bind(maximum.0.get())
        .bind(window_seconds.0.get())
        .fetch_one(pool.0)
        .await
        .map(|allowed| {
            if allowed {
                super::PgRateLimitDecision::Allowed
            } else {
                super::PgRateLimitDecision::Limited(window_seconds)
            }
        })
        .map_err(|error| super::PgRateLimitError::Sqlx(super::SqlxPgRateLimitError::from(error)))
}
