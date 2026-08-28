#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(crate) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}

impl AdminRateLimitScope {
    pub(crate) fn as_str(self) -> crate::StdAdminStrRef<'static> {
        match self {
            Self::AuditExport => {
                crate::StdAdminStrRef::from(constants_str::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT)
            }
            Self::Mutation => {
                crate::StdAdminStrRef::from(constants_str::SERVER_ADMIN_RATE_LIMIT_MUTATION)
            }
            Self::RefreshIp => {
                crate::StdAdminStrRef::from(constants_str::SERVER_ADMIN_RATE_LIMIT_REFRESH_IP)
            }
            Self::SignInIp => {
                crate::StdAdminStrRef::from(constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP)
            }
            Self::SignInIpLogin => {
                crate::StdAdminStrRef::from(constants_str::SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN)
            }
        }
    }
}
