#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(crate) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}

impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
    pub(crate) fn as_str(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        match self {
            Self::AuditExport => server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT,
            ),
            Self::Mutation => server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_MUTATION,
            ),
            Self::RefreshIp => server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_REFRESH_IP,
            ),
            Self::SignInIp => server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP,
            ),
            Self::SignInIpLogin => server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN,
            ),
        }
    }
}
