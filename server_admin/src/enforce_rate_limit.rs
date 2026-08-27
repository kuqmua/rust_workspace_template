pub(in crate::domain_types::auth) async fn enforce_rate_limit(
    state: &super::super::AdminAuthSvcState,
    scope: super::AdminRateLimitScope,
    subject: &super::super::super::StdAdminString,
    limit: super::super::StdAdminRateLimitCount,
    window_seconds: super::super::StdAdminRateLimitWindowSeconds,
) -> Result<(), super::super::AdminError> {
    let scope_text = scope.as_str();
    let decision = server_runtime_http::domain_types::enforce_pg_rate_limit(
        server_runtime_http::domain_types::SqlxPgRateLimitPoolRef::from(state.pool.as_ref()),
        server_runtime_http::domain_types::PgRateLimitQueryRef::from(
            constants_str::SERVER_ADMIN_ENFORCE_RATE_LIMIT_SQL,
        ),
        server_runtime_http::domain_types::PgRateLimitScopeRef::try_from(scope_text.as_ref())
            .map_err(|_error| super::super::AdminError::Validation)?,
        server_runtime_http::domain_types::PgRateLimitSubjectRef::try_from(
            subject.as_ref().as_str(),
        )
        .map_err(|_error| super::super::AdminError::Validation)?,
        server_runtime_http::domain_types::PgRateLimitMaximum::try_from(i64::from(limit))
            .map_err(|_error| super::super::AdminError::Validation)?,
        server_runtime_http::domain_types::PgRateLimitWindowSeconds::try_from(i32::from(
            window_seconds,
        ))
        .map_err(|_error| super::super::AdminError::Validation)?,
    )
    .await
    .map_err(|error| match error {
        server_runtime_http::domain_types::PgRateLimitError::Sqlx(source) => {
            super::super::AdminError::postgresql(super::super::super::SqlxAdminError::from(
                sqlx::Error::from(source),
            ))
        }
    })?;
    match decision {
        server_runtime_http::domain_types::PgRateLimitDecision::Allowed => Ok(()),
        server_runtime_http::domain_types::PgRateLimitDecision::Limited(_retry_after) => {
            Err(super::super::AdminError::RateLimited)
        }
    }
}
