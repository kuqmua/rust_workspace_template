use super::AdminDataTableStrRef;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::WireEnum,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(try_from = "String")]
#[wire_enum(
    ref_type = AdminDataTableStrRef,
    error_message = constants_str::UNKNOWN_ADMINISTRATOR_DATA_TABLE,
)]
pub enum AdminDataTable {
    #[wire("access_sessions")]
    AccessSessions,
    #[wire("audit_log")]
    AuditLog,
    #[wire("cleanup_status")]
    CleanupStatus,
    #[wire("login_attempts")]
    LoginAttempts,
    #[wire("permissions")]
    Permissions,
    #[wire("rate_limits")]
    RateLimits,
    #[wire("refresh_tokens")]
    RefreshTokens,
    #[wire("role_permissions")]
    RolePermissions,
    #[wire("roles")]
    Roles,
    #[wire("system_settings")]
    SystemSettings,
    #[wire("user_roles")]
    UserRoles,
    #[wire("users")]
    Users,
}

impl AdminDataTable {
    pub const PG_ORDER: [Self; 12] = [
        Self::Users,
        Self::Roles,
        Self::Permissions,
        Self::UserRoles,
        Self::RolePermissions,
        Self::RefreshTokens,
        Self::AccessSessions,
        Self::LoginAttempts,
        Self::AuditLog,
        Self::SystemSettings,
        Self::RateLimits,
        Self::CleanupStatus,
    ];

    #[must_use]
    pub fn supports_filters(self) -> crate::domain_types::AdminBool {
        self.spec().supports_filters()
    }

    #[must_use]
    pub fn frontend_path(self) -> crate::domain_types::AdminDataTableFrontendPath {
        crate::domain_types::AdminDataTableFrontendPath::from(self)
    }

    #[must_use]
    pub fn from_frontend_path(path: crate::domain_types::AdminPagePathRef<'_>) -> Option<Self> {
        let value = path
            .get()
            .strip_prefix(crate::domain_types::AdminFrontendPath::Root.get())
            .and_then(|value| value.strip_prefix('/'))
            .map(str::to_owned)?;
        Self::try_from(value).ok()
    }

    #[must_use]
    pub fn permission(self) -> super::AdminPermission {
        self.spec().permission()
    }

    #[must_use]
    pub fn spec(self) -> super::AdminDataTableSpec {
        match self {
            Self::AccessSessions => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                super::AdminPermission::AccessSessionsRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::AuditLog => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                super::AdminPermission::AuditLogRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::CleanupStatus => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_SINGLETON),
                super::AdminPermission::CleanupStatusRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::LoginAttempts => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_ATTEMPTED_AT),
                super::AdminPermission::LoginAttemptsRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::Permissions => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                super::AdminPermission::PermissionsRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::RateLimits => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_WINDOW),
                super::AdminPermission::RateLimitsRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::RefreshTokens => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                super::AdminPermission::RefreshTokensRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::RolePermissions => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                super::AdminPermission::RolePermissionsRead,
                crate::domain_types::AdminBool::from(true),
            ),
            Self::Roles => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_ROLES_COLUMNS),
                super::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                super::AdminPermission::RolesRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::SystemSettings => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                super::AdminPermission::SystemSettingsRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::UserRoles => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS,
                ),
                super::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                super::AdminPermission::UserRolesRead,
                crate::domain_types::AdminBool::from(false),
            ),
            Self::Users => super::AdminDataTableSpec::new(
                super::AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_USERS_COLUMNS),
                super::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                super::AdminPermission::UsersRead,
                crate::domain_types::AdminBool::from(false),
            ),
        }
    }
}

impl std::fmt::Display for AdminDataTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str().get())
    }
}

impl TryFrom<String> for AdminDataTable {
    type Error = AdminDataTableTryFromStrError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
