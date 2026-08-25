#![allow(clippy::arbitrary_source_item_ordering)] // DTO implementations keep constructors adjacent to their accessors and route metadata grouped by concern
pub const ADMIN_DISPLAY_NAME_MAX_CHARS: usize = 256usize;
pub const ADMIN_DISPLAY_NAME_MIN_CHARS: usize = 1usize;
pub const ADMIN_LOGIN_MAX_CHARS: usize = 128usize;
pub const ADMIN_LOGIN_MIN_CHARS: usize = 3usize;
pub const ADMIN_PASSWORD_MAX_CHARS: usize = 1024usize;
pub const ADMIN_PASSWORD_MIN_CHARS: usize = 1usize;
pub const ADMIN_NEW_PASSWORD_MIN_CHARS: usize = 12usize;
pub const ADMIN_ROLE_NAME_MAX_CHARS: usize = 128usize;
pub const ADMIN_ROLE_NAME_MIN_CHARS: usize = 1usize;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminApiBodyMaxBytes(usize);
pub(crate) const ADMIN_API_BODY_MAX_BYTES_VALUE: usize = 65_536usize;
#[must_use]
pub fn admin_api_body_max_bytes() -> AdminApiBodyMaxBytes {
    AdminApiBodyMaxBytes::from(ADMIN_API_BODY_MAX_BYTES_VALUE)
}
const ADMIN_DISPLAY_NAME_IS_VALID: fn(&str) -> bool = |value| value.trim() == value;
const ADMIN_LOGIN_IS_VALID: fn(&str) -> bool = |value| {
    value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'.' | b'-')
    })
};
const ADMIN_NEW_PASSWORD_IS_VALID: fn(&str) -> bool = |value| {
    text_policy::domain_types::validate_password_policy(
        text_policy::domain_types::PasswordTextRef::from(value),
        text_policy::domain_types::PasswordLengthRange::from_prevalidated(
            text_policy::domain_types::PasswordLength::from(ADMIN_NEW_PASSWORD_MIN_CHARS),
            text_policy::domain_types::PasswordLength::from(ADMIN_PASSWORD_MAX_CHARS),
        ),
    )
    .is_ok()
};
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    newtype::Display,
    newtype::FromInner,
)]
pub struct PositiveNonZeroI64(std::num::NonZeroI64);
impl utoipa::PartialSchema for PositiveNonZeroI64 {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::Integer)
            .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                utoipa::openapi::KnownFormat::Int64,
            )))
            .minimum(Some(1.0))
            .into()
    }
}
impl utoipa::ToSchema for PositiveNonZeroI64 {}
impl TryFrom<i64> for PositiveNonZeroI64 {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(value)
            .filter(|non_zero| non_zero.get().is_positive())
            .map(Self)
            .ok_or(AdminIdTryFromI64Error)
    }
}
impl PositiveNonZeroI64 {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
}
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
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator API text"
)]
pub struct AdminText(String);
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
    max = ADMIN_LOGIN_MAX_CHARS,
    min = ADMIN_LOGIN_MIN_CHARS,
    chars,
    serde,
    utoipa,
    validator = ADMIN_LOGIN_IS_VALID,
    description = "administrator login"
)]
pub struct AdminLogin(String);
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
    max = ADMIN_DISPLAY_NAME_MAX_CHARS,
    min = ADMIN_DISPLAY_NAME_MIN_CHARS,
    chars,
    serde,
    utoipa,
    validator = ADMIN_DISPLAY_NAME_IS_VALID,
    description = "administrator display name"
)]
pub struct AdminDisplayName(String);
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
    max = ADMIN_ROLE_NAME_MAX_CHARS,
    min = ADMIN_ROLE_NAME_MIN_CHARS,
    chars,
    serde,
    utoipa,
    validator = ADMIN_LOGIN_IS_VALID,
    description = "administrator role name"
)]
pub struct AdminRoleName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::BoundedString,
    newtype::DebugRedacted,
    newtype::IntoInner,
)]
#[bounded_string(
    max = ADMIN_PASSWORD_MAX_CHARS,
    min = ADMIN_PASSWORD_MIN_CHARS,
    chars,
    serde,
    utoipa,
    write_only,
    description = "administrator password"
)]
pub struct AdminPassword(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::BoundedString,
    newtype::DebugRedacted,
    newtype::IntoInner,
)]
#[bounded_string(
    max = ADMIN_PASSWORD_MAX_CHARS,
    min = ADMIN_NEW_PASSWORD_MIN_CHARS,
    chars,
    serde,
    utoipa,
    write_only,
    validator = ADMIN_NEW_PASSWORD_IS_VALID,
    description = "new administrator password"
)]
pub struct AdminNewPassword(String);
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
    supports_filters: AdminBool,
}
impl AdminDataTableSpec {
    const fn new(
        columns: AdminDataColumnsCsvRef<'static>,
        order: AdminDataOrderRef<'static>,
        permission: AdminPermission,
        supports_filters: AdminBool,
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
    pub const fn supports_filters(self) -> AdminBool {
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
    pub fn supports_filters(self) -> AdminBool {
        self.spec().supports_filters()
    }

    #[must_use]
    pub fn frontend_path(self) -> AdminDataTableFrontendPath {
        AdminDataTableFrontendPath::from(self)
    }

    #[must_use]
    pub fn from_frontend_path(path: AdminPagePathRef<'_>) -> Option<Self> {
        let value = path
            .get()
            .strip_prefix(AdminFrontendPath::Root.get())
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
                AdminBool::from(false),
            ),
            Self::AuditLog => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::AuditLogRead,
                AdminBool::from(false),
            ),
            Self::CleanupStatus => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_SINGLETON),
                AdminPermission::CleanupStatusRead,
                AdminBool::from(false),
            ),
            Self::LoginAttempts => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_ATTEMPTED_AT),
                AdminPermission::LoginAttemptsRead,
                AdminBool::from(false),
            ),
            Self::Permissions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::PermissionsRead,
                AdminBool::from(false),
            ),
            Self::RateLimits => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_WINDOW),
                AdminPermission::RateLimitsRead,
                AdminBool::from(false),
            ),
            Self::RefreshTokens => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_SESSION_COLUMNS),
                AdminDataOrderRef::from(constants_str::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::RefreshTokensRead,
                AdminBool::from(false),
            ),
            Self::RolePermissions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::RolePermissionsRead,
                AdminBool::from(true),
            ),
            Self::Roles => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_ROLES_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::RolesRead,
                AdminBool::from(false),
            ),
            Self::SystemSettings => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    constants_str::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
                ),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::SystemSettingsRead,
                AdminBool::from(false),
            ),
            Self::UserRoles => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::UserRolesRead,
                AdminBool::from(false),
            ),
            Self::Users => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(constants_str::SERVER_ADMIN_DATA_USERS_COLUMNS),
                AdminDataOrderRef::from(constants_str::SQL_NAMES_ID),
                AdminPermission::UsersRead,
                AdminBool::from(false),
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
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator audit timestamp"
)]
pub struct AdminAuditTimestamp(String);
pub const ADMIN_AUDIT_DETAILS_MAX_BYTES: usize = 4096usize;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
pub struct AdminAuditDetailsBytes(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(
    "administrator audit details contain {} bytes, maximum is {} bytes",
    .0.0,
    ADMIN_AUDIT_DETAILS_MAX_BYTES
)]
pub struct AdminAuditDetailsTooLarge(AdminAuditDetailsBytes);
impl AdminAuditDetailsTooLarge {
    #[must_use]
    pub const fn actual_bytes(self) -> AdminAuditDetailsBytes {
        self.0
    }
    #[must_use]
    pub fn maximum_bytes(self) -> AdminAuditDetailsBytes {
        AdminAuditDetailsBytes::from(ADMIN_AUDIT_DETAILS_MAX_BYTES)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerdeJsonAdminAuditDetails(serde_json::Value);
impl TryFrom<serde_json::Value> for SerdeJsonAdminAuditDetails {
    type Error = AdminAuditDetailsTooLarge;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let actual_bytes = value.to_string().len();
        if actual_bytes > ADMIN_AUDIT_DETAILS_MAX_BYTES {
            return Err(AdminAuditDetailsTooLarge(AdminAuditDetailsBytes::from(
                actual_bytes,
            )));
        }
        Ok(Self(value))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, chars, serde, utoipa, validator = |value: &String| { let path = AdminPagePathRef::from(value.as_str()); AdminPage::from_path(path).is_some() || AdminDataTable::from_frontend_path(path).is_some() }, description = "administrator default route")]
pub struct AdminDefaultRoute(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, min = constants_usize::ONE, chars, serde, utoipa, validator = |value: &String| !value
    .trim()
    .is_empty(), description = "administrator site name")]
pub struct AdminSiteName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator main logo"
)]
pub struct AdminMainLogo(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    chars,
    serde,
    utoipa,
    description = "administrator organization contacts"
)]
pub struct AdminOrganizationContacts(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    chars,
    serde,
    utoipa,
    description = "administrator organization name"
)]
pub struct AdminOrganizationName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = 7usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.len() == 7usize && value.bytes().next() == Some(b'#') && value.bytes().skip(constants_usize::ONE).all(|byte| byte.is_ascii_hexdigit()),
    description = "administrator primary color"
)]
pub struct AdminPrimaryColor(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator support URL"
)]
pub struct AdminSupportUrl(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| !value.trim().is_empty(),
    description = "administrator tab title"
)]
pub struct AdminTabTitle(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminTableSortField {
    AuditAction,
    AuditCreatedAt,
    AuditResource,
    AuditSucceeded,
    AuditUserId,
    PermissionId,
    PermissionName,
    RoleId,
    RoleName,
    RoleSystem,
    UserDisplayName,
    UserId,
    UserLogin,
    UserStatus,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq, thiserror::Error,
)]
#[error("{}", constants_str::UNKNOWN_ADMIN_TABLE_SORT_FIELD)]
pub struct AdminTableSortFieldTryFromKeyError;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AdminTableSortValues {
    key: frontend_contract::domain_types::ContractStr,
    label: frontend_contract::domain_types::ContractStr,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminTableSortKeyRef<'value_lt>(&'value_lt str);
impl AdminTableSortField {
    pub const USER: [Self; 4] = [
        Self::UserLogin,
        Self::UserDisplayName,
        Self::UserId,
        Self::UserStatus,
    ];
    pub const ROLE: [Self; 3] = [Self::RoleName, Self::RoleId, Self::RoleSystem];
    pub const PERMISSION: [Self; 2] = [Self::PermissionName, Self::PermissionId];
    pub const AUDIT: [Self; 5] = [
        Self::AuditCreatedAt,
        Self::AuditUserId,
        Self::AuditAction,
        Self::AuditResource,
        Self::AuditSucceeded,
    ];
    #[must_use]
    pub fn key(self) -> frontend_contract::domain_types::ContractStr {
        self.values().key
    }
    #[must_use]
    pub fn label(self) -> frontend_contract::domain_types::ContractStr {
        self.values().label
    }
    pub fn try_from_key(
        options: &[Self],
        key: AdminTableSortKeyRef<'_>,
    ) -> Result<Self, AdminTableSortFieldTryFromKeyError> {
        options
            .iter()
            .copied()
            .find(|option| option.key().as_ref() == key.0)
            .ok_or(AdminTableSortFieldTryFromKeyError)
    }
    fn values(self) -> AdminTableSortValues {
        let (key, label) = match self {
            Self::AuditAction => (constants_str::ACTION, constants_str::SHARED_VALUES_ACTION_2),
            Self::AuditCreatedAt => (constants_str::CREATED_AT, constants_str::SHARED_VALUES_TIME),
            Self::AuditResource => (
                constants_str::RESOURCE,
                constants_str::SHARED_VALUES_RESOURCE_2,
            ),
            Self::AuditSucceeded => (constants_str::SUCCEEDED, constants_str::RESULT),
            Self::AuditUserId => (constants_str::USER_ID, constants_str::SHARED_VALUES_USER),
            Self::PermissionId | Self::RoleId | Self::UserId => {
                (constants_str::SQL_NAMES_ID, constants_str::ID)
            }
            Self::PermissionName | Self::RoleName => {
                (constants_str::NAME, constants_str::SHARED_VALUES_NAME_2)
            }
            Self::RoleSystem => (constants_str::SYSTEM, constants_str::SHARED_VALUES_SYSTEM_2),
            Self::UserDisplayName => (
                constants_str::DISPLAY_NAME,
                constants_str::SHARED_VALUES_DISPLAY_NAME_2,
            ),
            Self::UserLogin => (constants_str::LOGIN, constants_str::SHARED_VALUES_LOGIN_2),
            Self::UserStatus => (
                constants_str::STATUS_ALT,
                constants_str::SHARED_VALUES_STATUS_2,
            ),
        };
        AdminTableSortValues {
            key: frontend_contract::domain_types::ContractStr::from(key),
            label: frontend_contract::domain_types::ContractStr::from(label),
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminUserId(PositiveNonZeroI64);
impl TryFrom<i64> for AdminUserId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminUserId> for i64 {
    fn from(value: AdminUserId) -> Self {
        value.0.get()
    }
}
impl AdminUserId {
    #[must_use]
    pub const fn value(self) -> PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminRoleId(PositiveNonZeroI64);
impl TryFrom<i64> for AdminRoleId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminRoleId> for i64 {
    fn from(value: AdminRoleId) -> Self {
        value.0.get()
    }
}
impl AdminRoleId {
    #[must_use]
    pub const fn value(self) -> PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminPermissionId(PositiveNonZeroI64);
impl TryFrom<i64> for AdminPermissionId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminPermissionId> for i64 {
    fn from(value: AdminPermissionId) -> Self {
        value.0.get()
    }
}
impl AdminPermissionId {
    #[must_use]
    pub const fn value(self) -> PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminAuditLogId(PositiveNonZeroI64);
impl TryFrom<i64> for AdminAuditLogId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminAuditLogId> for i64 {
    fn from(value: AdminAuditLogId) -> Self {
        value.0.get()
    }
}
impl AdminAuditLogId {
    #[must_use]
    pub const fn value(self) -> PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminIdTryFromI64Error;
mod query;
pub use query::{
    AdminBool, AdminDataTableFilterQuery, AdminDataTableQuery, AdminFilterField,
    AdminFilterOperationKey, AdminFilterValue, AdminPageLimit, AdminPageLimitError,
    AdminPageOffset, AdminPageTotal, AdminSortDirection, AdminTableQuery, AdminTableSearch,
    AdminTableSortKey,
};

mod collections;
#[cfg(test)]
use collections::ADMIN_COLLECTION_MAX_ITEMS;
pub use collections::{
    AdminAuditViews, AdminCollectionError, AdminDataRows, AdminDataTables, AdminOptionalSettings,
    AdminPermissionIds, AdminPermissionSummaries, AdminPermissionValues, AdminRoleIds,
    AdminRoleNames, AdminRoleSummaries, AdminSessionViews, AdminTexts, AdminUserSummaries,
};
mod dto;
pub use dto::{
    AdminAuditCursor, AdminAuditExport, AdminAuditExportCsv, AdminAuditPage, AdminAuditView,
    AdminChangeOwnPasswordReq, AdminCreateRoleReq, AdminCreateRoleRes, AdminCreateUserReq,
    AdminCreateUserRes, AdminDataColumn, AdminDataColumns, AdminDataFilter, AdminDataFilters,
    AdminDataInputKind, AdminDataRow, AdminDataTableCatalog, AdminDataTableView,
    AdminPermissionSummary, AdminPermissionsPage, AdminRoleSummary, AdminRolesPage,
    AdminSetRolePermissionsReq, AdminSetUserBanReq, AdminSetUserPasswordReq, AdminSetUserRolesReq,
    AdminSignInReq, AdminSignInRes, AdminUpdateRoleReq, AdminUpdateUserReq, AdminUserSummary,
    AdminUsersPage, AuthenticatedAdmin,
};

mod settings;
pub use settings::{
    AdminBrandingView, AdminOptionalSetting, AdminSetting, AdminSettingInputKind,
    AdminSettingLabel, AdminSettingName, AdminSettingOptionality, AdminSettingSpec,
    AdminSettingsView, AdminUpdateSettingsReq,
};

mod sessions;
pub use sessions::{
    AdminNoBody, AdminSessionIdentifier, AdminSessionTimestamp, AdminSessionView, AdminSessionsPage,
};

mod routes;
pub use routes::{
    AdminAuditExportRoute, AdminAuditLogRoute, AdminAuthenticationRouteFamily, AdminBrandingRoute,
    AdminChangeOwnPasswordRoute, AdminCreateRoleRoute, AdminCreateUserRoute,
    AdminDataTableFrontendPath, AdminDataTableRoute, AdminDataTablesRoute, AdminDeleteRoleRoute,
    AdminDeleteUserRoute, AdminFrontendPath, AdminHtmlAction, AdminListPermissionsRoute,
    AdminListRolesRoute, AdminListUsersRoute, AdminMeRoute, AdminPage, AdminPageCapability,
    AdminPageClientMode, AdminPageMetadata, AdminPageNavigation, AdminPagePathRef, AdminPageSpec,
    AdminRefreshRoute, AdminRevokeAllSessionsRoute, AdminRevokeSessionRoute, AdminRoute,
    AdminRoutePath, AdminRoutePathError, AdminSessionsRoute, AdminSetRolePermissionsRoute,
    AdminSetUserBanRoute, AdminSetUserPasswordRoute, AdminSetUserRolesRoute, AdminSettingsRoute,
    AdminSignInRoute, AdminSignOutRoute, AdminUpdateRoleRoute, AdminUpdateSettingsRoute,
    AdminUpdateUserRoute, admin_parameterized_route_path, audit_log_client, audit_log_route,
    branding_client, branding_route, change_own_password_client, change_own_password_route,
    create_role_client, create_role_route, create_user_client, create_user_route,
    delete_role_client, delete_role_route, delete_user_client, delete_user_route,
    export_audit_log_client, export_audit_log_route, list_data_tables_client,
    list_data_tables_route, list_permissions_client, list_permissions_route, list_roles_client,
    list_roles_route, list_users_client, list_users_route, me_client, me_route, metrics_client,
    metrics_route, open_api_client, open_api_route, read_data_table_client, read_data_table_route,
    refresh_client, refresh_route, revoke_all_sessions_client, revoke_all_sessions_route,
    revoke_session_client, revoke_session_route, sessions_client, sessions_route,
    set_role_permissions_client, set_role_permissions_route, set_user_ban_client,
    set_user_ban_route, set_user_password_client, set_user_password_route, set_user_roles_client,
    set_user_roles_route, settings_client, settings_route, sign_in_client, sign_in_route,
    sign_out_client, sign_out_route, update_role_client, update_role_route, update_settings_client,
    update_settings_route, update_user_client, update_user_route, version_client, version_route,
};
#[cfg(test)]
mod tests;
