pub(crate) async fn enforce_rate_limit(
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    admin_rate_limit_scope: crate::admin_rate_limit_scope::AdminRateLimitScope,
    std_admin_string: &server_admin_core::std_admin_string::StdAdminString,
    std_admin_rate_limit_count: &crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    std_admin_rate_limit_window_seconds: &crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
) -> Result<(), crate::admin_error::AdminError> {
    let scope_text = admin_rate_limit_scope.as_str();
    let decision = server_runtime_http::enforce_pg_rate_limit::enforce_pg_rate_limit(
        server_runtime_http::sqlx_pg_rate_limit_pool_ref::SqlxPgRateLimitPoolRef::from(
            admin_auth_svc_state.get_pool().as_ref(),
        ),
        server_runtime_http::pg_rate_limit_query_ref::PgRateLimitQueryRef::from(
            constants_str::SERVER_ADMIN_ENFORCE_RATE_LIMIT_SQL,
        ),
        server_runtime_http::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(
            scope_text.as_ref(),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        server_runtime_http::pg_rate_limit_subject_ref::PgRateLimitSubjectRef::try_from(
            std_admin_string.as_ref().as_str(),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        server_runtime_http::pg_rate_limit_maximum::PgRateLimitMaximum::try_from(i64::from(
            *std_admin_rate_limit_count,
        ))
        .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        server_runtime_http::pg_rate_limit_window_seconds::PgRateLimitWindowSeconds::try_from(
            i32::from(*std_admin_rate_limit_window_seconds),
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
