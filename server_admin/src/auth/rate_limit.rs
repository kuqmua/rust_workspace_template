#![allow(clippy::field_scoped_visibility_modifiers)] // auth state reads the validated count while the private module owns construction and enforcement
#[derive(Debug, Clone, Copy)]
pub(super) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}
impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // scope serialization is shared by persistence and exhaustive contract tests
    pub(super) fn as_str(self) -> super::super::StdAdminStrRef<'static> {
        match self {
            Self::AuditExport => super::super::StdAdminStrRef::from(
                str_constants::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT,
            ),
            Self::Mutation => {
                super::super::StdAdminStrRef::from(str_constants::SERVER_ADMIN_RATE_LIMIT_MUTATION)
            }
            Self::RefreshIp => super::super::StdAdminStrRef::from(
                str_constants::SERVER_ADMIN_RATE_LIMIT_REFRESH_IP,
            ),
            Self::SignInIp => super::super::StdAdminStrRef::from(
                str_constants::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP,
            ),
            Self::SignInIpLogin => super::super::StdAdminStrRef::from(
                str_constants::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN,
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
) -> Result<(), super::AdminApiError> {
    let scope_text = scope.as_str();
    let decision = super::super::repository::rate_limits::enforce_rate_limit(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
        scope_text,
        subject,
        limit,
        window_seconds,
    )
    .await
    .map_err(|repository_error| match repository_error {
        super::super::repository::AdminRateLimitRepositoryError::InvalidPolicy => {
            super::AdminApiError::Validation
        }
        super::super::repository::AdminRateLimitRepositoryError::Sqlx(sqlx_error) => {
            super::AdminApiError::pg(sqlx_error)
        }
    })?;
    match decision {
        super::super::repository::AdminRateLimitOutcome::Allowed => Ok(()),
        super::super::repository::AdminRateLimitOutcome::Limited => {
            Err(super::AdminApiError::RateLimited)
        }
    }
}
