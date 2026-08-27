#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(in crate::domain_types::auth) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}

impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // production enforcement and the exhaustive test each use this mapping in different targets
    pub(in crate::domain_types::auth) fn as_str(
        self,
    ) -> super::super::super::StdAdminStrRef<'static> {
        match self {
            Self::AuditExport => super::super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT,
            ),
            Self::Mutation => super::super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_MUTATION,
            ),
            Self::RefreshIp => super::super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_REFRESH_IP,
            ),
            Self::SignInIp => super::super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP,
            ),
            Self::SignInIpLogin => super::super::super::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN,
            ),
        }
    }
}
