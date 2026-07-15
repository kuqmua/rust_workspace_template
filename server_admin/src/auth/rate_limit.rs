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
                super::super::StdAdminStrRef(str_constants::server_admin::RATE_LIMIT_AUDIT_READ)
            }
            Self::Mutation => {
                super::super::StdAdminStrRef(str_constants::server_admin::RATE_LIMIT_MUTATION)
            }
            Self::RefreshIp => {
                super::super::StdAdminStrRef(str_constants::server_admin::RATE_LIMIT_REFRESH_IP)
            }
            Self::SignInIp => {
                super::super::StdAdminStrRef(str_constants::server_admin::RATE_LIMIT_SIGN_IN_IP)
            }
            Self::SignInIpLogin => super::super::StdAdminStrRef(
                str_constants::server_admin::RATE_LIMIT_SIGN_IN_IP_LOGIN,
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
    let allowed = sqlx::query_scalar::<_, bool>(str_constants::expr::S_0685)
        .bind(scope.as_str().as_ref())
        .bind(subject.as_ref())
        .bind(limit.0)
        .bind(window_seconds.0)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|error| super::AdminApiError::Pg(super::super::SqlxAdminError::from(error)))?;
    if allowed {
        Ok(())
    } else {
        Err(super::AdminApiError::RateLimited)
    }
}
