pub async fn enforce_pg_rate_limit(
    pool: crate::sqlx_pg_rate_limit_pool_ref::SqlxPgRateLimitPoolRef<'_>,
    query: crate::pg_rate_limit_query_ref::PgRateLimitQueryRef,
    scope: crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef<'_>,
    subject: crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef<'_>,
    maximum: crate::pg_rate_limit_maximum::PgRateLimitMaximum,
    window_seconds: crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds,
) -> Result<
    crate::pg_rate_limit_decision::PgRateLimitDecision,
    crate::pg_rate_limit_error::PgRateLimitError,
> {
    sqlx::query_scalar::<_, bool>(query.get())
        .bind(scope.get())
        .bind(subject.get())
        .bind(maximum.get().get())
        .bind(window_seconds.get().get())
        .fetch_one(pool.get())
        .await
        .map(|allowed| {
            if allowed {
                crate::pg_rate_limit_decision::PgRateLimitDecision::Allowed
            } else {
                crate::pg_rate_limit_decision::PgRateLimitDecision::Limited(window_seconds)
            }
        })
        .map_err(|error| {
            crate::pg_rate_limit_error::PgRateLimitError::Sqlx(
                crate::sqlx_pg_rate_limit_error::SqlxPgRateLimitError::from(error),
            )
        })
}
