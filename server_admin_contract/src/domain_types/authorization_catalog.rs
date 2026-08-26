#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInner,
)]
#[bounded_string(
    max = 128,
    chars,
    serde,
    utoipa,
    description = "administrator permission"
)]
pub struct AdminPermissionValue(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminPermissionStrRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::WireEnum,
    utoipa::ToSchema,
)]
#[wire_enum(
    ref_type = AdminPermissionStrRef,
    error_message = constants_str::UNKNOWN_ADMINISTRATOR_PERMISSION,
)]
pub enum AdminPermission {
    #[wire("access_sessions:read")]
    AccessSessionsRead,
    #[wire("audit_log:export")]
    AuditLogExport,
    #[wire("audit_log:read")]
    AuditLogRead,
    #[wire("cleanup_status:read")]
    CleanupStatusRead,
    #[wire("login_attempts:read")]
    LoginAttemptsRead,
    #[wire("metrics:read")]
    MetricsRead,
    #[wire("openapi:read")]
    OpenApiRead,
    #[wire("permissions:read")]
    PermissionsRead,
    #[wire("rate_limits:read")]
    RateLimitsRead,
    #[wire("refresh_tokens:read")]
    RefreshTokensRead,
    #[wire("role_permissions:create")]
    RolePermissionsCreate,
    #[wire("role_permissions:delete")]
    RolePermissionsDelete,
    #[wire("role_permissions:read")]
    RolePermissionsRead,
    #[wire("role_permissions:update")]
    RolePermissionsUpdate,
    #[wire("roles:create")]
    RolesCreate,
    #[wire("roles:delete")]
    RolesDelete,
    #[wire("roles:read")]
    RolesRead,
    #[wire("roles:update")]
    RolesUpdate,
    #[wire("system_settings:read")]
    SystemSettingsRead,
    #[wire("system_settings:update")]
    SystemSettingsUpdate,
    #[wire("tables:read")]
    TablesRead,
    #[wire("user_roles:create")]
    UserRolesCreate,
    #[wire("user_roles:delete")]
    UserRolesDelete,
    #[wire("user_roles:read")]
    UserRolesRead,
    #[wire("user_roles:update")]
    UserRolesUpdate,
    #[wire("users:create")]
    UsersCreate,
    #[wire("users:delete")]
    UsersDelete,
    #[wire("users:read")]
    UsersRead,
    #[wire("users:update")]
    UsersUpdate,
}

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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminDataTableStrRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminDataColumnsCsvRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminDataOrderRef<'value_lt>(&'value_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminDataTableSpec {
    columns: AdminDataColumnsCsvRef<'static>,
    order: AdminDataOrderRef<'static>,
    permission: AdminPermission,
    supports_filters: super::AdminBool,
}
impl AdminDataTableSpec {
    const fn new(
        columns: AdminDataColumnsCsvRef<'static>,
        order: AdminDataOrderRef<'static>,
        permission: AdminPermission,
        supports_filters: super::AdminBool,
    ) -> Self {
        Self {
            columns,
            order,
            permission,
            supports_filters,
        }
    }
    #[must_use]
    pub const fn columns(self) -> AdminDataColumnsCsvRef<'static> {
        self.columns
    }
    #[must_use]
    pub const fn order(self) -> AdminDataOrderRef<'static> {
        self.order
    }
    #[must_use]
    pub const fn permission(self) -> AdminPermission {
        self.permission
    }
    #[must_use]
    pub const fn supports_filters(self) -> super::AdminBool {
        self.supports_filters
    }
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
    pub fn supports_filters(self) -> super::AdminBool {
        self.spec().supports_filters()
    }

    #[must_use]
    pub fn frontend_path(self) -> super::AdminDataTableFrontendPath {
        super::AdminDataTableFrontendPath::from(self)
    }

    #[must_use]
    pub fn from_frontend_path(path: super::AdminPagePathRef<'_>) -> Option<Self> {
        let value = path
            .get()
            .strip_prefix(super::AdminFrontendPath::Root.get())
            .and_then(|value| value.strip_prefix('/'))
            .map(str::to_owned)?;
        Self::try_from(value).ok()
    }

    #[must_use]
    pub fn permission(self) -> AdminPermission {
        self.spec().permission()
    }

    #[must_use]
    pub fn spec(self) -> AdminDataTableSpec {
        match self {
            Self::AccessSessions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::AccessSessionsRead,
                super::AdminBool::from(false),
            ),
            Self::AuditLog => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::AuditLogRead,
                super::AdminBool::from(false),
            ),
            Self::CleanupStatus => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_SINGLETON),
                AdminPermission::CleanupStatusRead,
                super::AdminBool::from(false),
            ),
            Self::LoginAttempts => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_ATTEMPTED_AT),
                AdminPermission::LoginAttemptsRead,
                super::AdminBool::from(false),
            ),
            Self::Permissions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::PermissionsRead,
                super::AdminBool::from(false),
            ),
            Self::RateLimits => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_WINDOW),
                AdminPermission::RateLimitsRead,
                super::AdminBool::from(false),
            ),
            Self::RefreshTokens => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::RefreshTokensRead,
                super::AdminBool::from(false),
            ),
            Self::RolePermissions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::RolePermissionsRead,
                super::AdminBool::from(true),
            ),
            Self::Roles => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_ROLES_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::RolesRead,
                super::AdminBool::from(false),
            ),
            Self::SystemSettings => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::SystemSettingsRead,
                super::AdminBool::from(false),
            ),
            Self::UserRoles => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::UserRolesRead,
                super::AdminBool::from(false),
            ),
            Self::Users => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_USERS_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::UsersRead,
                super::AdminBool::from(false),
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

#[cfg(test)]
mod tests {
    #[test]
    fn user_table_requires_user_read_permission() {
        assert_eq!(
            super::AdminDataTable::Users.permission(),
            super::AdminPermission::UsersRead,
        );
    }
}
