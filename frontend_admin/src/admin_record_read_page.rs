#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub(crate) enum AdminRecordReadPage {
    AccessSession,
    AuditLog,
    CleanupStatus,
    LoginAttempt,
    PermissionAction,
    PermissionResourceAction,
    PermissionResource,
    RateLimit,
    RefreshToken,
    Rule,
    RoleRule,
    SystemSetting,
    UserRole,
    User,
}
impl AdminRecordReadPage {
    pub(crate) const fn data_page(self) -> &'static str {
        match self {
            Self::AccessSession => constants_str::ADMIN_ACCESS_SESSION_READ_PAGE,
            Self::AuditLog => constants_str::ADMIN_AUDIT_LOG_READ_PAGE,
            Self::CleanupStatus => constants_str::ADMIN_CLEANUP_STATUS_READ_PAGE,
            Self::LoginAttempt => constants_str::ADMIN_LOGIN_ATTEMPT_READ_PAGE,
            Self::PermissionAction => constants_str::ADMIN_PERMISSION_ACTION_READ_PAGE,
            Self::PermissionResourceAction => {
                constants_str::ADMIN_PERMISSION_RESOURCE_ACTION_READ_PAGE
            }
            Self::PermissionResource => constants_str::ADMIN_PERMISSION_RESOURCE_READ_PAGE,
            Self::RateLimit => constants_str::ADMIN_RATE_LIMIT_READ_PAGE,
            Self::RefreshToken => constants_str::ADMIN_REFRESH_TOKEN_READ_PAGE,
            Self::Rule => constants_str::ADMIN_RULE_READ_PAGE,
            Self::RoleRule => constants_str::ADMIN_ROLE_RULE_READ_PAGE,
            Self::SystemSetting => constants_str::ADMIN_SYSTEM_SETTING_READ_PAGE,
            Self::UserRole => constants_str::ADMIN_USER_ROLE_READ_PAGE,
            Self::User => constants_str::ADMIN_USER_READ_PAGE,
        }
    }
}
