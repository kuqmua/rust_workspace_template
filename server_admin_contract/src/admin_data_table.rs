#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_wire_enum::WireEnum,
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
    #[wire("permission_actions")]
    PermissionActions,
    #[wire("permission_resource_actions")]
    PermissionResourceActions,
    #[wire("permission_resources")]
    PermissionResources,
    #[wire("rules")]
    Rules,
    #[wire("rate_limits")]
    RateLimits,
    #[wire("refresh_tokens")]
    RefreshTokens,
    #[wire("role_rules")]
    RoleRules,
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
    #[must_use]
    pub const fn api_route(self) -> crate::admin_route::AdminRoute {
        match self {
            Self::UserRoles => crate::admin_route::AdminRoute::UserRolesTable,
            Self::RoleRules => crate::admin_route::AdminRoute::RoleRulesTable,
            Self::RefreshTokens => crate::admin_route::AdminRoute::RefreshTokensTable,
            Self::AccessSessions => crate::admin_route::AdminRoute::AccessSessionsTable,
            Self::LoginAttempts => crate::admin_route::AdminRoute::LoginAttemptsTable,
            Self::PermissionActions => crate::admin_route::AdminRoute::PermissionActions,
            Self::PermissionResourceActions => {
                crate::admin_route::AdminRoute::PermissionResourceActions
            }
            Self::PermissionResources => crate::admin_route::AdminRoute::PermissionResources,
            Self::RateLimits => crate::admin_route::AdminRoute::RateLimitsTable,
            Self::CleanupStatus => crate::admin_route::AdminRoute::CleanupStatusTable,
            Self::Users => crate::admin_route::AdminRoute::Users,
            Self::Roles => crate::admin_route::AdminRoute::Roles,
            Self::Rules => crate::admin_route::AdminRoute::Rules,
            Self::AuditLog => crate::admin_route::AdminRoute::AuditLog,
            Self::SystemSettings => crate::admin_route::AdminRoute::SystemSettings,
        }
    }

    pub const PG_ORDER: [Self; 15] = [
        Self::Users,
        Self::UserRoles,
        Self::Roles,
        Self::RoleRules,
        Self::Rules,
        Self::PermissionActions,
        Self::PermissionResourceActions,
        Self::PermissionResources,
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
        admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        let value = admin_page_path_ref
            .get()
            .strip_prefix(crate::admin_frontend_path::AdminFrontendPath::Root.get())
            .and_then(|value| value.strip_prefix('/'))
            .map(str::to_owned)?;
        Self::try_from(value).ok()
    }

    #[must_use]
    pub fn rule(self) -> crate::admin_rule::AdminRule {
        self.spec().rule()
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
                crate::admin_rule::AdminRule::AccessSessionsRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::AuditLog => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT,
                ),
                crate::admin_rule::AdminRule::AuditLogRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::CleanupStatus => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_CLEANUP_STATUS_ID,
                ),
                crate::admin_rule::AdminRule::CleanupStatusRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::LoginAttempts => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_ATTEMPTED_AT,
                ),
                crate::admin_rule::AdminRule::LoginAttemptsRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::PermissionActions => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_PERMISSION_ACTIONS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::PermissionActionsRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::PermissionResourceActions => {
                crate::admin_data_table_spec::AdminDataTableSpec::new(
                    crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                        constants_str::SERVER_ADMIN_DATA_PERMISSION_RESOURCE_ACTIONS_COLUMNS,
                    ),
                    crate::admin_data_order_ref::AdminDataOrderRef::from(
                        constants_str::SQL_NAMES_ID,
                    ),
                    crate::admin_rule::AdminRule::PermissionResourceActionsRead,
                    crate::admin_bool::AdminBool::from(true),
                )
            }
            Self::PermissionResources => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_PERMISSION_RESOURCES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::PermissionResourcesRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::Rules => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_RULES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::RulesRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::RateLimits => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_RATE_LIMITS_ID,
                ),
                crate::admin_rule::AdminRule::RateLimitsRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::RefreshTokens => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(
                    constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT,
                ),
                crate::admin_rule::AdminRule::RefreshTokensRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::RoleRules => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLE_RULES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::RoleRulesRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::Roles => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::RolesRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::SystemSettings => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::SystemSettingsRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::UserRoles => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::UserRolesRead,
                crate::admin_bool::AdminBool::from(true),
            ),
            Self::Users => crate::admin_data_table_spec::AdminDataTableSpec::new(
                crate::admin_data_columns_csv_ref::AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_USERS_COLUMNS,
                ),
                crate::admin_data_order_ref::AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                crate::admin_rule::AdminRule::UsersRead,
                crate::admin_bool::AdminBool::from(true),
            ),
        }
    }
}

impl std::fmt::Display for AdminDataTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str().get())
    }
}

impl TryFrom<String> for AdminDataTable {
    type Error = AdminDataTableTryFromStrError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
