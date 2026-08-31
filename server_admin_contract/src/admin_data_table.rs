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
    ref_type = crate::admin_data_table_str_ref::AdminDataTableStrRef,
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
    pub fn supports_filters(self) -> crate::admin_bool::AdminBool {
        self.spec().supports_filters()
    }

    #[must_use]
    pub fn frontend_path(
        self,
    ) -> crate::admin_data_table_frontend_path::AdminDataTableFrontendPath {
        crate::admin_data_table_frontend_path::AdminDataTableFrontendPath::from(self)
    }

    #[must_use]
    pub fn from_frontend_path(
        path: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        let value = path
            .get()
            .strip_prefix(crate::admin_frontend_path::AdminFrontendPath::Root.get())
            .and_then(|value| value.strip_prefix('/'))
            .map(str::to_owned)?;
        Self::try_from(value).ok()
    }

    #[must_use]
    pub fn permission(self) -> crate::admin_permission::AdminPermission {
        self.spec().permission()
    }

    #[must_use]
    pub fn spec(self) -> crate::admin_data_table_spec::AdminDataTableSpec {
        match self {
            Self::AccessSessions => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT,
                ),
                crate::admin_permission::AdminPermission::AccessSessionsRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::AuditLog => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT,
                ),
                crate::admin_permission::AdminPermission::AuditLogRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::CleanupStatus => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_SINGLETON,
                ),
                crate::admin_permission::AdminPermission::CleanupStatusRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::LoginAttempts => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_ATTEMPTED_AT,
                ),
                crate::admin_permission::AdminPermission::LoginAttemptsRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::Permissions => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_permission::AdminPermission::PermissionsRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::RateLimits => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_WINDOW,
                ),
                crate::admin_permission::AdminPermission::RateLimitsRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::RefreshTokens => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT,
                ),
                crate::admin_permission::AdminPermission::RefreshTokensRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::RolePermissions => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_permission::AdminPermission::RolePermissionsRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::Roles => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_permission::AdminPermission::RolesRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::SystemSettings => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_permission::AdminPermission::SystemSettingsRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::UserRoles => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_permission::AdminPermission::UserRolesRead,
                crate::admin_bool::AdminBool::from(false),
            ),
            Self::Users => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_USERS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_permission::AdminPermission::UsersRead,
                crate::admin_bool::AdminBool::from(false),
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
