#![allow(clippy::field_scoped_visibility_modifiers)] // auth state reads the validated count while the private module owns construction and enforcement
#[derive(Debug, Clone, Copy)]
pub(super) enum AdminRateLimitScope {
    AuditRead,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}
impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // scope serialization is shared by persistence and exhaustive contract tests
    pub(super) const fn as_str(self) -> super::super::StdAdminStrRef<'static> {
        match self {
            Self::AuditRead => {
                super::super::StdAdminStrRef(str_constants::SERVER_ADMIN_RATE_LIMIT_AUDIT_READ)
            }
            Self::Mutation => {
                super::super::StdAdminStrRef(str_constants::SERVER_ADMIN_RATE_LIMIT_MUTATION)
            }
            Self::RefreshIp => {
                super::super::StdAdminStrRef(str_constants::SERVER_ADMIN_RATE_LIMIT_REFRESH_IP)
            }
            Self::SignInIp => {
                super::super::StdAdminStrRef(str_constants::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP)
            }
            Self::SignInIpLogin => super::super::StdAdminStrRef(
                str_constants::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN,
            ),
        }
    }
}
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub(super) struct StdAdminRateLimitCount(pub(super) i64);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub(super) struct StdAdminRateLimitWindowSeconds(i32);
pub(super) async fn enforce_rate_limit(
    state: &super::AdminAuthSvcState,
    scope: AdminRateLimitScope,
    subject: &super::super::StdAdminString,
    limit: StdAdminRateLimitCount,
    window_seconds: StdAdminRateLimitWindowSeconds,
) -> Result<(), super::AdminApiError> {
    let scope_text = scope.as_str();
    let decision = server_runtime::enforce_pg_rate_limit(
        server_runtime::SqlxPgRateLimitPoolRef::from(state.pool.as_ref()),
        server_runtime::PgRateLimitQueryRef::from(
            str_constants::INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT,
        ),
        server_runtime::PgRateLimitScopeRef::try_from(scope_text.as_ref())
            .map_err(|_error| super::AdminApiError::Validation)?,
        server_runtime::PgRateLimitSubjectRef::try_from(subject.as_ref().as_str())
            .map_err(|_error| super::AdminApiError::Validation)?,
        server_runtime::PgRateLimitMaximum::try_from(limit.0)
            .map_err(|_error| super::AdminApiError::Validation)?,
        server_runtime::PgRateLimitWindowSeconds::try_from(window_seconds.0)
            .map_err(|_error| super::AdminApiError::Validation)?,
    )
    .await
    .map_err(|error| match error {
        server_runtime::PgRateLimitError::Sqlx(source) => super::AdminApiError::Pg(
            super::super::SqlxAdminError::from(sqlx::Error::from(source)),
        ),
    })?;
    match decision {
        server_runtime::PgRateLimitDecision::Allowed => Ok(()),
        server_runtime::PgRateLimitDecision::Limited(_retry_after) => {
            Err(super::AdminApiError::RateLimited)
        }
    }
}
