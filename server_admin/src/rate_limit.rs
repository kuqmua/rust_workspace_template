#![allow(clippy::field_scoped_visibility_modifiers)] // auth state reads the validated count while the private module owns construction and enforcement
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(super) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}
impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // production enforcement and the exhaustive test each use this mapping in different targets
    pub(super) fn as_str(self) -> super::super::StdAdminStrRef<'static> {
        match self {
            Self::AuditExport => super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT,
            ),
            Self::Mutation => {
                super::super::StdAdminStrRef::from(constants_str::SERVER_ADMIN_RATE_LIMIT_MUTATION)
            }
            Self::RefreshIp => super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_REFRESH_IP,
            ),
            Self::SignInIp => super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP,
            ),
            Self::SignInIpLogin => super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN,
            ),
        }
    }
}
pub(super) async fn enforce_rate_limit(
    state: &super::AdminAuthSvcState,
    scope: AdminRateLimitScope,
    subject: &super::super::StdAdminString,
    limit: super::StdAdminRateLimitCount,
    window_seconds: super::StdAdminRateLimitWindowSeconds,
) -> Result<(), super::AdminError> {
    let scope_text = scope.as_str();
    let decision = server_runtime_http::domain_types::enforce_pg_rate_limit(
        server_runtime_http::domain_types::SqlxPgRateLimitPoolRef::from(state.pool.as_ref()),
        server_runtime_http::domain_types::PgRateLimitQueryRef::from(
            constants_str::SERVER_ADMIN_ENFORCE_RATE_LIMIT_SQL,
        ),
        server_runtime_http::domain_types::PgRateLimitScopeRef::try_from(scope_text.as_ref())
            .map_err(|_error| super::AdminError::Validation)?,
        server_runtime_http::domain_types::PgRateLimitSubjectRef::try_from(
            subject.as_ref().as_str(),
        )
        .map_err(|_error| super::AdminError::Validation)?,
        server_runtime_http::domain_types::PgRateLimitMaximum::try_from(i64::from(limit))
            .map_err(|_error| super::AdminError::Validation)?,
        server_runtime_http::domain_types::PgRateLimitWindowSeconds::try_from(i32::from(
            window_seconds,
        ))
        .map_err(|_error| super::AdminError::Validation)?,
    )
    .await
    .map_err(|error| match error {
        server_runtime_http::domain_types::PgRateLimitError::Sqlx(source) => {
            super::AdminError::postgresql(super::super::SqlxAdminError::from(sqlx::Error::from(
                source,
            )))
        }
    })?;
    match decision {
        server_runtime_http::domain_types::PgRateLimitDecision::Allowed => Ok(()),
        server_runtime_http::domain_types::PgRateLimitDecision::Limited(_retry_after) => {
            Err(super::AdminError::RateLimited)
        }
    }
}
