pub(crate) async fn enforce_rate_limit(
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    scope: crate::admin_rate_limit_scope::AdminRateLimitScope,
    subject: &server_admin_core::std_admin_string::StdAdminString,
    limit: &crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    window_seconds: &crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
) -> Result<(), crate::admin_error::AdminError> {
    let scope_text = scope.as_str();
    let decision = server_runtime_http::enforce_pg_rate_limit::enforce_pg_rate_limit(
        server_runtime_http::sqlx_pg_rate_limit_pool_ref::SqlxPgRateLimitPoolRef::from(
            state.get_pool().as_ref(),
        ),
        server_runtime_http::pg_rate_limit_query_ref::PgRateLimitQueryRef::from(
            constants_str::SERVER_ADMIN_ENFORCE_RATE_LIMIT_SQL,
        ),
        server_runtime_http::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(
            scope_text.as_ref(),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        server_runtime_http::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(
            subject.as_ref().as_str(),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        server_runtime_http::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(i64::from(*limit))
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        server_runtime_http::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds::try_from(
            i32::from(*window_seconds),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?,
    )
    .await
    .map_err(|error| match error {
        server_runtime_http::pg_rate_limit_error::PgRateLimitError::Sqlx(source) => {
            crate::admin_error::AdminError::postgresql(
                crate::sqlx_admin_error::SqlxAdminError::from(sqlx::Error::from(source)),
            )
        }
    })?;
    match decision {
        server_runtime_http::pg_rate_limit_decision::PgRateLimitDecision::Allowed => Ok(()),
        server_runtime_http::pg_rate_limit_decision::PgRateLimitDecision::Limited(_retry_after) => {
            Err(crate::admin_error::AdminError::RateLimited)
        }
    }
}
