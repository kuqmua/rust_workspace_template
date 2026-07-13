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
            Self::AuditRead => super::super::StdAdminStrRef("audit_read"),
            Self::Mutation => super::super::StdAdminStrRef("mutation"),
            Self::RefreshIp => super::super::StdAdminStrRef("refresh_ip"),
            Self::SignInIp => super::super::StdAdminStrRef("sign_in_ip"),
            Self::SignInIpLogin => super::super::StdAdminStrRef("sign_in_ip_login"),
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
) -> Result<(), super::AdminApiEr> {
    let allowed = sqlx::query_scalar::<_, bool>("INSERT INTO admin_rate_limits (scope, subject, window_started_at, request_count) VALUES ($1, $2, NOW(), 1) ON CONFLICT (scope, subject) DO UPDATE SET window_started_at = CASE WHEN admin_rate_limits.window_started_at <= NOW() - make_interval(secs => $4) THEN NOW() ELSE admin_rate_limits.window_started_at END, request_count = CASE WHEN admin_rate_limits.window_started_at <= NOW() - make_interval(secs => $4) THEN 1 ELSE admin_rate_limits.request_count + 1 END RETURNING request_count <= $3")
        .bind(scope.as_str().as_ref())
        .bind(subject.as_ref())
        .bind(limit.0)
        .bind(window_seconds.0)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|er| super::AdminApiEr::Pg(super::super::SqlxAdminEr::from(er)))?;
    if allowed {
        Ok(())
    } else {
        Err(super::AdminApiEr::RateLimited)
    }
}
