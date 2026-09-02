pub async fn enforce_pg_rate_limit(
    sqlx_pg_rate_limit_pool_ref: crate::sqlx_pg_rate_limit_pool_ref::SqlxPgRateLimitPoolRef<'_>,
    pg_rate_limit_query_ref: crate::pg_rate_limit_query_ref::PgRateLimitQueryRef,
    pg_rate_limit_scope_ref: crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef<'_>,
    pg_rate_limit_subject_ref: crate::pg_rate_limit_subject_ref::PgRateLimitSubjectRef<'_>,
    pg_rate_limit_maximum: crate::pg_rate_limit_maximum::PgRateLimitMaximum,
    pg_rate_limit_window_seconds: crate::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds,
) -> Result<
    crate::pg_rate_limit_decision::PgRateLimitDecision,
    crate::pg_rate_limit_error::PgRateLimitError,
> {
    sqlx::query_scalar::<_, bool>(pg_rate_limit_query_ref.get())
        .bind(pg_rate_limit_scope_ref.get())
        .bind(pg_rate_limit_subject_ref.get())
        .bind(pg_rate_limit_maximum.get().get())
        .bind(pg_rate_limit_window_seconds.get().get())
        .fetch_one(sqlx_pg_rate_limit_pool_ref.get())
        .await
        .map(|allowed| {
            if allowed {
                crate::pg_rate_limit_decision::PgRateLimitDecision::Allowed
            } else {
                crate::pg_rate_limit_decision::PgRateLimitDecision::Limited(
                    pg_rate_limit_window_seconds,
                )
            }
        })
        .map_err(|error| {
            crate::pg_rate_limit_error::PgRateLimitError::Sqlx(
                crate::sqlx_pg_rate_limit_error::SqlxPgRateLimitError::from(error),
            )
        })
}
