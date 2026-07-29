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
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct AdminApiBodyMaxBytes(usize);
impl AdminApiBodyMaxBytes {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
const ADMIN_API_BODY_MAX_BYTES_VALUE: usize = 65_536usize;
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
    text_policy::validate_password_policy(
        text_policy::PasswordTextRef::from(value),
        text_policy::PasswordLengthRange::from_prevalidated(
            text_policy::PasswordLength::from(ADMIN_NEW_PASSWORD_MIN_CHARS),
            text_policy::PasswordLength::from(ADMIN_PASSWORD_MAX_CHARS),
        ),
    )
    .is_ok()
};
#[derive(
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
pub struct StdAdminPositiveI64(std::num::NonZeroI64);
impl utoipa::PartialSchema for StdAdminPositiveI64 {
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
impl utoipa::ToSchema for StdAdminPositiveI64 {}
impl TryFrom<i64> for StdAdminPositiveI64 {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(value)
            .filter(|non_zero| non_zero.get().is_positive())
            .map(Self)
            .ok_or(AdminIdTryFromI64Error)
    }
}
impl StdAdminPositiveI64 {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
}
#[derive(
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner)]
pub struct AdminPermissionStrRef<'value_lt>(&'value_lt str);
impl<'value_lt> AdminPermissionStrRef<'value_lt> {
    #[must_use]
    pub const fn get(self) -> &'value_lt str {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::WireEnum, utoipa::ToSchema)]
#[wire_enum(
    ref_type = AdminPermissionStrRef,
    error_message = str_constants::UNKNOWN_ADMINISTRATOR_PERMISSION,
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
    Debug, Clone, Copy, PartialEq, Eq, newtype::WireEnum, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(try_from = "String")]
#[wire_enum(
    ref_type = AdminDataTableStrRef,
    error_message = str_constants::UNKNOWN_ADMINISTRATOR_DATA_TABLE,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner)]
pub struct AdminDataTableStrRef<'value_lt>(&'value_lt str);
impl<'value_lt> AdminDataTableStrRef<'value_lt> {
    #[must_use]
    pub const fn get(self) -> &'value_lt str {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner)]
pub struct AdminDataColumnsCsvRef<'value_lt>(&'value_lt str);
impl<'value_lt> AdminDataColumnsCsvRef<'value_lt> {
    #[must_use]
    pub const fn get(self) -> &'value_lt str {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner)]
pub struct AdminDataOrderRef<'value_lt>(&'value_lt str);
impl<'value_lt> AdminDataOrderRef<'value_lt> {
    #[must_use]
    pub const fn get(self) -> &'value_lt str {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            .0
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
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_SESSION_COLUMNS),
                AdminDataOrderRef::from(str_constants::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::AccessSessionsRead,
                AdminBool::from(false),
            ),
            Self::AuditLog => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS),
                AdminDataOrderRef::from(str_constants::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::AuditLogRead,
                AdminBool::from(false),
            ),
            Self::CleanupStatus => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    str_constants::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
                ),
                AdminDataOrderRef::from(str_constants::SERVER_ADMIN_DATA_ORDER_SINGLETON),
                AdminPermission::CleanupStatusRead,
                AdminBool::from(false),
            ),
            Self::LoginAttempts => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    str_constants::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
                ),
                AdminDataOrderRef::from(str_constants::SERVER_ADMIN_DATA_ORDER_ATTEMPTED_AT),
                AdminPermission::LoginAttemptsRead,
                AdminBool::from(false),
            ),
            Self::Permissions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS),
                AdminDataOrderRef::from(str_constants::SQL_NAMES_ID),
                AdminPermission::PermissionsRead,
                AdminBool::from(false),
            ),
            Self::RateLimits => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS),
                AdminDataOrderRef::from(str_constants::SERVER_ADMIN_DATA_ORDER_WINDOW),
                AdminPermission::RateLimitsRead,
                AdminBool::from(false),
            ),
            Self::RefreshTokens => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_SESSION_COLUMNS),
                AdminDataOrderRef::from(str_constants::SERVER_ADMIN_DATA_ORDER_CREATED_AT),
                AdminPermission::RefreshTokensRead,
                AdminBool::from(false),
            ),
            Self::RolePermissions => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    str_constants::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
                ),
                AdminDataOrderRef::from(str_constants::SQL_NAMES_ID),
                AdminPermission::RolePermissionsRead,
                AdminBool::from(true),
            ),
            Self::Roles => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_ROLES_COLUMNS),
                AdminDataOrderRef::from(str_constants::SQL_NAMES_ID),
                AdminPermission::RolesRead,
                AdminBool::from(false),
            ),
            Self::SystemSettings => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(
                    str_constants::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
                ),
                AdminDataOrderRef::from(str_constants::SQL_NAMES_ID),
                AdminPermission::SystemSettingsRead,
                AdminBool::from(false),
            ),
            Self::UserRoles => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS),
                AdminDataOrderRef::from(str_constants::SQL_NAMES_ID),
                AdminPermission::UserRolesRead,
                AdminBool::from(false),
            ),
            Self::Users => AdminDataTableSpec::new(
                AdminDataColumnsCsvRef::from(str_constants::SERVER_ADMIN_DATA_USERS_COLUMNS),
                AdminDataOrderRef::from(str_constants::SQL_NAMES_ID),
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, newtype::FromInner)]
pub struct AdminAuditDetailsBytes(usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error, newtype::FromInner)]
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
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(max = 8_192usize, chars, serde, utoipa, validator = |value: &String| { let path = AdminPagePathRef::from(value.as_str()); AdminPage::from_path(path).is_some() || AdminDataTable::from_frontend_path(path).is_some() }, description = "administrator default route")]
pub struct AdminDefaultRoute(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(max = 8192usize, min = 1usize, chars, serde, utoipa, validator = |value: &String| !value
    .trim()
    .is_empty(), description = "administrator site name")]
pub struct AdminSiteName(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8_192usize,
    min = 1usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator main logo"
)]
pub struct AdminMainLogo(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8_192usize,
    chars,
    serde,
    utoipa,
    description = "administrator organization contacts"
)]
pub struct AdminOrganizationContacts(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8_192usize,
    chars,
    serde,
    utoipa,
    description = "administrator organization name"
)]
pub struct AdminOrganizationName(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8_192usize,
    min = 7usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.len() == 7usize && value.bytes().next() == Some(b'#') && value.bytes().skip(1usize).all(|byte| byte.is_ascii_hexdigit()),
    description = "administrator primary color"
)]
pub struct AdminPrimaryColor(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8_192usize,
    min = 1usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator support URL"
)]
pub struct AdminSupportUrl(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8_192usize,
    min = 1usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| !value.trim().is_empty(),
    description = "administrator tab title"
)]
pub struct AdminTabTitle(String);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{}", str_constants::UNKNOWN_ADMIN_TABLE_SORT_FIELD)]
pub struct AdminTableSortFieldTryFromKeyError;
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
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
    pub fn key(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(match self {
            Self::AuditAction => str_constants::ACTION,
            Self::AuditCreatedAt => str_constants::CREATED_AT,
            Self::AuditResource => str_constants::RESOURCE,
            Self::AuditSucceeded => str_constants::SUCCEEDED,
            Self::AuditUserId => str_constants::USER_ID,
            Self::PermissionId | Self::RoleId | Self::UserId => str_constants::SQL_NAMES_ID,
            Self::PermissionName | Self::RoleName => str_constants::NAME,
            Self::RoleSystem => str_constants::SYSTEM,
            Self::UserDisplayName => str_constants::DISPLAY_NAME,
            Self::UserLogin => str_constants::LOGIN,
            Self::UserStatus => str_constants::STATUS_ALT,
        })
    }
    #[must_use]
    pub fn label(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(match self {
            Self::AuditAction => str_constants::SHARED_VALUES_ACTION_2,
            Self::AuditCreatedAt => str_constants::SHARED_VALUES_TIME,
            Self::AuditResource => str_constants::SHARED_VALUES_RESOURCE_2,
            Self::AuditSucceeded => str_constants::RESULT,
            Self::AuditUserId => str_constants::SHARED_VALUES_USER,
            Self::PermissionId | Self::RoleId | Self::UserId => str_constants::ID,
            Self::PermissionName | Self::RoleName => str_constants::SHARED_VALUES_NAME_2,
            Self::RoleSystem => str_constants::SHARED_VALUES_SYSTEM_2,
            Self::UserDisplayName => str_constants::SHARED_VALUES_DISPLAY_NAME_2,
            Self::UserLogin => str_constants::SHARED_VALUES_LOGIN_2,
            Self::UserStatus => str_constants::SHARED_VALUES_STATUS_2,
        })
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
}
#[derive(
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
pub struct AdminUserId(StdAdminPositiveI64);
impl TryFrom<i64> for AdminUserId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        StdAdminPositiveI64::try_from(value).map(Self)
    }
}
impl From<AdminUserId> for i64 {
    fn from(value: AdminUserId) -> Self {
        value.0.get()
    }
}
impl AdminUserId {
    #[must_use]
    pub const fn value(self) -> StdAdminPositiveI64 {
        self.0
    }
}
#[derive(
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
pub struct AdminRoleId(StdAdminPositiveI64);
impl TryFrom<i64> for AdminRoleId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        StdAdminPositiveI64::try_from(value).map(Self)
    }
}
impl From<AdminRoleId> for i64 {
    fn from(value: AdminRoleId) -> Self {
        value.0.get()
    }
}
impl AdminRoleId {
    #[must_use]
    pub const fn value(self) -> StdAdminPositiveI64 {
        self.0
    }
}
#[derive(
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
pub struct AdminPermissionId(StdAdminPositiveI64);
impl TryFrom<i64> for AdminPermissionId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        StdAdminPositiveI64::try_from(value).map(Self)
    }
}
impl From<AdminPermissionId> for i64 {
    fn from(value: AdminPermissionId) -> Self {
        value.0.get()
    }
}
impl AdminPermissionId {
    #[must_use]
    pub const fn value(self) -> StdAdminPositiveI64 {
        self.0
    }
}
#[derive(
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
pub struct AdminAuditLogId(StdAdminPositiveI64);
impl TryFrom<i64> for AdminAuditLogId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        StdAdminPositiveI64::try_from(value).map(Self)
    }
}
impl From<AdminAuditLogId> for i64 {
    fn from(value: AdminAuditLogId) -> Self {
        value.0.get()
    }
}
impl AdminAuditLogId {
    #[must_use]
    pub const fn value(self) -> StdAdminPositiveI64 {
        self.0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub struct AdminIdTryFromI64Error;
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[serde(from = "bool")]
pub struct AdminBool(bool);

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct AdminPageOffset(u32);
struct AdminPageOffsetVisitor;
impl serde::de::Visitor<'_> for AdminPageOffsetVisitor {
    type Value = AdminPageOffset;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(str_constants::ADMIN_PAGE_OFFSET_EXPECTING)
    }
    fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        v.parse::<u32>()
            .map(AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        u32::try_from(v)
            .map(AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
}
impl<'de> serde::Deserialize<'de> for AdminPageOffset {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_any(AdminPageOffsetVisitor)?;
        Ok(Self::from(u32::from(value)))
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, newtype::IntoInnerFrom, serde::Serialize, utoipa::ToSchema,
)]
pub struct AdminPageLimit(u16);
struct AdminPageLimitVisitor;
impl serde::de::Visitor<'_> for AdminPageLimitVisitor {
    type Value = AdminPageLimit;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an administrator page limit from {} through {}",
            AdminPageLimit::MIN,
            AdminPageLimit::MAX
        )
    }
    fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = v.parse::<u16>().map_err(serde::de::Error::custom)?;
        AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = u16::try_from(v).map_err(serde::de::Error::custom)?;
        AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
}
impl<'de> serde::Deserialize<'de> for AdminPageLimit {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_any(AdminPageLimitVisitor)?;
        Self::try_from(u16::from(value)).map_err(serde::de::Error::custom)
    }
}
struct AdminDefaultPageLimit;
impl From<AdminDefaultPageLimit> for AdminPageLimit {
    fn from(_value: AdminDefaultPageLimit) -> Self {
        Self(Self::DEFAULT)
    }
}
impl Default for AdminPageLimit {
    fn default() -> Self {
        Self::from(AdminDefaultPageLimit)
    }
}
impl TryFrom<u16> for AdminPageLimit {
    type Error = AdminPageLimitError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminPageLimitError)
        }
    }
}
impl AdminPageLimit {
    pub const DEFAULT: u16 = 20u16;
    pub const MAX: u16 = 100u16;
    pub const MIN: u16 = 1u16;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error(
    "administrator page limit must be between {min} and {max}",
    min = AdminPageLimit::MIN,
    max = AdminPageLimit::MAX
)]
pub struct AdminPageLimitError;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[serde(from = "u64")]
pub struct AdminPageTotal(u64);

#[derive(Clone, Debug, Default, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 128usize,
    chars,
    serde,
    utoipa,
    description = "administrator table search"
)]
pub struct AdminTableSearch(String);

#[derive(Clone, Debug, Default, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 32usize,
    chars,
    serde,
    utoipa,
    description = "administrator table sort key"
)]
pub struct AdminTableSortKey(String);

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminSortDirection {
    #[default]
    Asc,
    Desc,
}
impl AsRef<str> for AdminSortDirection {
    fn as_ref(&self) -> &str {
        match self {
            Self::Asc => str_constants::ASC_ALT,
            Self::Desc => str_constants::DESC_ALT,
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
pub struct AdminTableQuery {
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: AdminPageLimit,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: AdminPageOffset,
    #[serde(default)]
    #[param(value_type = String, max_length = 128)]
    search: AdminTableSearch,
    #[serde(default)]
    #[param(value_type = String, max_length = 32)]
    sort: AdminTableSortKey,
    #[serde(default)]
    #[param(inline)]
    direction: AdminSortDirection,
}
impl AdminTableQuery {
    #[must_use]
    pub fn pagination(limit: AdminPageLimit, offset: AdminPageOffset) -> Self {
        Self {
            limit,
            offset,
            ..Self::default()
        }
    }
    #[must_use]
    pub const fn limit(&self) -> AdminPageLimit {
        self.limit
    }
    #[must_use]
    pub const fn offset(&self) -> AdminPageOffset {
        self.offset
    }
    #[must_use]
    pub const fn search(&self) -> &AdminTableSearch {
        &self.search
    }
    #[must_use]
    pub const fn sort(&self) -> &AdminTableSortKey {
        &self.sort
    }
    #[must_use]
    pub const fn direction(&self) -> AdminSortDirection {
        self.direction
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 63usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter field"
)]
pub struct AdminFilterField(String);
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 4096usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter value"
)]
pub struct AdminFilterValue(String);
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = 63usize)]
pub struct AdminFilterOperationKey(String);
impl From<frontend_contract::FilterOperation> for AdminFilterOperationKey {
    fn from(value: frontend_contract::FilterOperation) -> Self {
        let mut key = String::new();
        format!("{value:?}")
            .chars()
            .enumerate()
            .for_each(|(index, character)| {
                if character.is_uppercase() && index > 0usize {
                    key.push('_');
                }
                key.extend(character.to_lowercase());
            });
        Self::try_from(key).unwrap_or_default()
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
pub struct AdminDataTableFilterQuery {
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 63)]
    filter_field: Option<AdminFilterField>,
    #[serde(default)]
    #[param(inline)]
    filter_operation: Option<frontend_contract::FilterOperation>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    filter_value: Option<AdminFilterValue>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    filter_end: Option<AdminFilterValue>,
}
impl AdminDataTableFilterQuery {
    #[must_use]
    pub const fn new(
        filter_field: Option<AdminFilterField>,
        filter_operation: Option<frontend_contract::FilterOperation>,
        filter_value: Option<AdminFilterValue>,
        filter_end: Option<AdminFilterValue>,
    ) -> Self {
        Self {
            filter_field,
            filter_operation,
            filter_value,
            filter_end,
        }
    }
    #[must_use]
    pub const fn field(&self) -> Option<&AdminFilterField> {
        self.filter_field.as_ref()
    }
    #[must_use]
    pub const fn operation(&self) -> Option<frontend_contract::FilterOperation> {
        self.filter_operation
    }
    #[must_use]
    pub const fn value(&self) -> Option<&AdminFilterValue> {
        self.filter_value.as_ref()
    }
    #[must_use]
    pub const fn end(&self) -> Option<&AdminFilterValue> {
        self.filter_end.as_ref()
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct AdminDataTableQuery {
    #[serde(flatten)]
    filter: AdminDataTableFilterQuery,
    #[serde(flatten)]
    page: AdminTableQuery,
}
impl utoipa::IntoParams for AdminDataTableQuery {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        let parameter_in = parameter_in_provider();
        let mut parameters =
            <AdminDataTableFilterQuery as utoipa::IntoParams>::into_params(|| parameter_in.clone());
        parameters.extend(<AdminTableQuery as utoipa::IntoParams>::into_params(|| {
            parameter_in.clone()
        }));
        parameters
    }
}
impl AdminDataTableQuery {
    #[must_use]
    pub const fn new(filter: AdminDataTableFilterQuery, page: AdminTableQuery) -> Self {
        Self { filter, page }
    }
    #[must_use]
    pub const fn filter(&self) -> &AdminDataTableFilterQuery {
        &self.filter
    }
    #[must_use]
    pub const fn page(&self) -> &AdminTableQuery {
        &self.page
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminSignInReq {
    login: AdminLogin,
    password: AdminPassword,
}
impl AdminSignInReq {
    #[must_use]
    pub const fn new(login: AdminLogin, password: AdminPassword) -> Self {
        Self { login, password }
    }
    #[must_use]
    pub const fn login(&self) -> &AdminLogin {
        &self.login
    }
    #[must_use]
    pub fn into_parts(self) -> (AdminLogin, AdminPassword) {
        (self.login, self.password)
    }
    #[must_use]
    pub const fn password(&self) -> &AdminPassword {
        &self.password
    }
}
const ADMIN_COLLECTION_MAX_ITEMS: usize = 10_000usize;
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdminCollectionError {
    #[error(
        "{}",
        str_constants::ADMINISTRATOR_COLLECTION_EXCEEDS_MAXIMUM_ITEM_COUNT
    )]
    TooLong,
}
#[derive(Clone, Debug, newtype::DerefTarget, newtype::IntoInnerFrom)]
struct AdminBoundedVec<T>(Vec<T>);
impl<T> AdminBoundedVec<T> {
    const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T> From<[T; 0]> for AdminBoundedVec<T> {
    fn from(_value: [T; 0]) -> Self {
        Self(Vec::new())
    }
}
impl<T> TryFrom<Vec<T>> for AdminBoundedVec<T> {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() > ADMIN_COLLECTION_MAX_ITEMS {
            Err(AdminCollectionError::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(newtype::FromInner)]
struct StdPhantomDataAdminBoundedVecVisitor<T>(std::marker::PhantomData<T>);
impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de>
    for StdPhantomDataAdminBoundedVecVisitor<T>
{
    type Value = AdminBoundedVec<T>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an administrator array with at most {ADMIN_COLLECTION_MAX_ITEMS} items"
        )
    }
    fn visit_seq<Seq>(self, mut seq: Seq) -> Result<Self::Value, Seq::Error>
    where
        Seq: serde::de::SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(
            seq.size_hint()
                .unwrap_or_default()
                .min(ADMIN_COLLECTION_MAX_ITEMS),
        );
        while let Some(value) = seq.next_element()? {
            if values.len() == ADMIN_COLLECTION_MAX_ITEMS {
                return Err(serde::de::Error::custom(AdminCollectionError::TooLong));
            }
            values.push(value);
        }
        AdminBoundedVec::try_from(values).map_err(serde::de::Error::custom)
    }
}
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for AdminBoundedVec<T> {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(StdPhantomDataAdminBoundedVecVisitor::from(
            std::marker::PhantomData,
        ))
    }
}
impl<T: serde::Serialize> serde::Serialize for AdminBoundedVec<T> {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}
#[allow(dead_code)] // schema-only generic carries its item type without runtime construction
struct AdminOpenApiVec<T, const MAX: usize> {
    marker: StdPhantomDataAdminBoundedVecVisitor<T>,
}
impl<T: utoipa::PartialSchema, const MAX: usize> utoipa::__dev::ComposeSchema
    for AdminOpenApiVec<T, MAX>
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .max_items(Some(MAX))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema, const MAX: usize> utoipa::ToSchema for AdminOpenApiVec<T, MAX> {
    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        schemas.push((
            T::name().into_owned(),
            <T as utoipa::PartialSchema>::schema(),
        ));
        T::schemas(schemas);
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminPermissionValue>")]
#[schema(value_type = AdminOpenApiVec<AdminPermissionValue, 10_000>)]
pub struct AdminPermissionValues(AdminBoundedVec<AdminPermissionValue>);
impl TryFrom<Vec<AdminPermissionValue>> for AdminPermissionValues {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminPermissionValue>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminRoleName>")]
#[schema(value_type = AdminOpenApiVec<AdminRoleName, 10_000>)]
pub struct AdminRoleNames(AdminBoundedVec<AdminRoleName>);
impl TryFrom<Vec<AdminRoleName>> for AdminRoleNames {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminRoleName>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminRoleId>")]
#[schema(value_type = AdminOpenApiVec<AdminRoleId, 10_000>)]
pub struct AdminRoleIds(AdminBoundedVec<AdminRoleId>);
impl TryFrom<Vec<AdminRoleId>> for AdminRoleIds {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminRoleId>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminPermissionId>")]
#[schema(value_type = AdminOpenApiVec<AdminPermissionId, 10_000>)]
pub struct AdminPermissionIds(AdminBoundedVec<AdminPermissionId>);
impl TryFrom<Vec<AdminPermissionId>> for AdminPermissionIds {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminPermissionId>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminUserSummary>")]
#[schema(value_type = AdminOpenApiVec<AdminUserSummary, 10_000>)]
pub struct AdminUserSummaries(AdminBoundedVec<AdminUserSummary>);
impl TryFrom<Vec<AdminUserSummary>> for AdminUserSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminUserSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminRoleSummary>")]
#[schema(value_type = AdminOpenApiVec<AdminRoleSummary, 10_000>)]
pub struct AdminRoleSummaries(AdminBoundedVec<AdminRoleSummary>);
impl TryFrom<Vec<AdminRoleSummary>> for AdminRoleSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminRoleSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminPermissionSummary>")]
#[schema(value_type = AdminOpenApiVec<AdminPermissionSummary, 10_000>)]
pub struct AdminPermissionSummaries(AdminBoundedVec<AdminPermissionSummary>);
impl TryFrom<Vec<AdminPermissionSummary>> for AdminPermissionSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminPermissionSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminAuditView>")]
#[schema(value_type = AdminOpenApiVec<AdminAuditView, 10_000>)]
pub struct AdminAuditViews(AdminBoundedVec<AdminAuditView>);
impl TryFrom<Vec<AdminAuditView>> for AdminAuditViews {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminAuditView>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminText>")]
#[schema(value_type = AdminOpenApiVec<AdminText, 10_000>)]
pub struct AdminTexts(AdminBoundedVec<AdminText>);
impl TryFrom<Vec<AdminText>> for AdminTexts {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminText>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminDataRow>")]
#[schema(value_type = AdminOpenApiVec<AdminDataRow, 10_000>)]
pub struct AdminDataRows(AdminBoundedVec<AdminDataRow>);
impl TryFrom<Vec<AdminDataRow>> for AdminDataRows {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminDataRow>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminDataTable>")]
#[schema(value_type = AdminOpenApiVec<AdminDataTable, 10_000>)]
pub struct AdminDataTables(AdminBoundedVec<AdminDataTable>);
impl TryFrom<Vec<AdminDataTable>> for AdminDataTables {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminDataTable>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminOptionalSetting>")]
#[schema(value_type = AdminOpenApiVec<AdminOptionalSetting, 10_000>)]
pub struct AdminOptionalSettings(AdminBoundedVec<AdminOptionalSetting>);
impl TryFrom<Vec<AdminOptionalSetting>> for AdminOptionalSettings {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminOptionalSetting>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminSessionView>")]
#[schema(value_type = AdminOpenApiVec<AdminSessionView, 10_000>)]
pub struct AdminSessionViews(AdminBoundedVec<AdminSessionView>);
impl TryFrom<Vec<AdminSessionView>> for AdminSessionViews {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminSessionView>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminRoleIds {
    fn default() -> Self {
        Self::from(AdminEmptyCollection)
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminPermissionIds {
    fn default() -> Self {
        Self::from(AdminEmptyCollection)
    }
}
struct AdminEmptyCollection;
impl From<AdminEmptyCollection> for AdminRoleIds {
    fn from(_value: AdminEmptyCollection) -> Self {
        Self(AdminBoundedVec::from([]))
    }
}
impl From<AdminEmptyCollection> for AdminPermissionIds {
    fn from(_value: AdminEmptyCollection) -> Self {
        Self(AdminBoundedVec::from([]))
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    display_name: AdminDisplayName,
    id: AdminUserId,
    login: AdminLogin,
    permissions: AdminPermissionValues,
    roles: AdminRoleNames,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn new(
        display_name: AdminDisplayName,
        id: AdminUserId,
        login: AdminLogin,
        permissions: AdminPermissionValues,
        roles: AdminRoleNames,
    ) -> Self {
        Self {
            display_name,
            id,
            login,
            permissions,
            roles,
        }
    }
    #[must_use]
    pub const fn display_name(&self) -> &AdminDisplayName {
        &self.display_name
    }
    #[must_use]
    pub fn permissions(&self) -> &[AdminPermissionValue] {
        self.permissions.as_ref()
    }
    #[must_use]
    pub const fn login(&self) -> &AdminLogin {
        &self.login
    }
    #[must_use]
    pub const fn roles(&self) -> &[AdminRoleName] {
        self.roles.0.as_slice()
    }
    #[must_use]
    pub fn has_permission(&self, permission: AdminPermission) -> AdminBool {
        let required = permission.as_str();
        AdminBool::from(
            self.permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.get()),
        )
    }
    #[must_use]
    pub fn can_access(&self, page: AdminPage) -> AdminBool {
        AdminBool::from(match page.authentication() {
            frontend_contract::AuthenticationRequirement::Authenticated
            | frontend_contract::AuthenticationRequirement::Public => true,
            frontend_contract::AuthenticationRequirement::Permission(required) => self
                .permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.as_ref()),
        })
    }
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSignInRes {
    #[contract_struct_api(borrow)]
    user: AuthenticatedAdmin,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateUserReq {
    display_name: AdminDisplayName,
    login: AdminLogin,
    password: AdminNewPassword,
}
#[derive(
    Clone,
    Copy,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminCreateUserRes {
    id: AdminUserId,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateUserReq {
    display_name: Option<AdminDisplayName>,
    login: Option<AdminLogin>,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserPasswordReq {
    #[contract_struct_api(into)]
    password: AdminNewPassword,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminChangeOwnPasswordReq {
    current_password: AdminPassword,
    new_password: AdminNewPassword,
}
#[derive(
    Clone,
    Copy,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserBanReq {
    #[contract_struct_api(copy)]
    is_banned: AdminBool,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateRoleReq {
    #[contract_struct_api(into)]
    name: AdminRoleName,
}
#[derive(
    Clone,
    Copy,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminCreateRoleRes {
    id: AdminRoleId,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateRoleReq {
    #[contract_struct_api(into)]
    name: AdminRoleName,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserRolesReq {
    expected_role_ids: AdminRoleIds,
    role_ids: AdminRoleIds,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsReq {
    expected_permission_ids: AdminPermissionIds,
    permission_ids: AdminPermissionIds,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminUserSummary {
    #[contract_struct_api(borrow)]
    display_name: AdminDisplayName,
    #[contract_struct_api(copy_ref)]
    id: AdminUserId,
    #[contract_struct_api(copy_ref)]
    is_banned: AdminBool,
    #[contract_struct_api(borrow)]
    login: AdminLogin,
    #[serde(default)]
    #[contract_struct_api(slice = AdminRoleId)]
    role_ids: AdminRoleIds,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminRoleSummary {
    #[contract_struct_api(copy_ref)]
    id: AdminRoleId,
    #[contract_struct_api(copy_ref)]
    is_system: AdminBool,
    name: AdminRoleName,
    #[serde(default)]
    #[contract_struct_api(slice = AdminPermissionId)]
    permission_ids: AdminPermissionIds,
}
impl AdminRoleSummary {
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &AdminRoleName {
        &self.name
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminPermissionSummary {
    id: AdminPermissionId,
    name: AdminPermissionValue,
}
impl AdminPermissionSummary {
    #[must_use]
    pub const fn new(id: AdminPermissionId, name: AdminPermissionValue) -> Self {
        Self { id, name }
    }
    #[must_use]
    pub const fn id(&self) -> AdminPermissionId {
        self.id
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &AdminPermissionValue {
        &self.name
    }
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminUsersPage {
    #[contract_struct_api(into, slice = AdminUserSummary)]
    items: AdminUserSummaries,
    #[contract_struct_api(slice = AdminRoleSummary)]
    roles: AdminRoleSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: AdminPageTotal,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminRolesPage {
    #[contract_struct_api(into, slice = AdminRoleSummary)]
    items: AdminRoleSummaries,
    #[contract_struct_api(slice = AdminPermissionSummary)]
    permissions: AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: AdminPageTotal,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminPermissionsPage {
    #[contract_struct_api(into, slice = AdminPermissionSummary)]
    items: AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: AdminPageTotal,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditView {
    #[contract_struct_api(borrow)]
    action: AdminText,
    #[contract_struct_api(borrow)]
    created_at: AdminAuditTimestamp,
    #[contract_struct_api(option_borrow)]
    details: Option<SerdeJsonAdminAuditDetails>,
    #[contract_struct_api(copy_ref)]
    id: AdminAuditLogId,
    #[contract_struct_api(borrow)]
    resource: AdminText,
    #[contract_struct_api(option_borrow)]
    resource_id: Option<AdminText>,
    #[contract_struct_api(copy_ref)]
    succeeded: AdminBool,
    #[contract_struct_api(copy_ref)]
    user_id: Option<AdminUserId>,
    #[contract_struct_api(option_borrow)]
    user_login: Option<AdminLogin>,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditCursor {
    #[contract_struct_api(borrow)]
    created_at: AdminAuditTimestamp,
    #[contract_struct_api(copy_ref)]
    id: AdminAuditLogId,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditPage {
    #[contract_struct_api(slice = AdminAuditView)]
    items: AdminAuditViews,
    #[schema(inline)]
    #[contract_struct_api(option_borrow)]
    next_cursor: Option<AdminAuditCursor>,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: AdminPageTotal,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminDataColumn {
    filters: AdminDataFilters,
    input_kind: AdminDataInputKind,
    label: AdminText,
    name: AdminText,
}
impl AdminDataColumn {
    #[must_use]
    pub const fn new(
        filters: AdminDataFilters,
        input_kind: AdminDataInputKind,
        label: AdminText,
        name: AdminText,
    ) -> Self {
        Self {
            filters,
            input_kind,
            label,
            name,
        }
    }
    #[must_use]
    pub const fn filters(&self) -> &[AdminDataFilter] {
        self.filters.as_slice()
    }
    #[must_use]
    pub const fn input_kind(&self) -> AdminDataInputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(&self) -> &AdminText {
        &self.label
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &AdminText {
        &self.name
    }
}
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
pub struct AdminDataFilter {
    operation: frontend_contract::FilterOperation,
    value_shape: frontend_contract::FilterValueShape,
}
impl From<frontend_contract::FilterOperation> for AdminDataFilter {
    fn from(value: frontend_contract::FilterOperation) -> Self {
        Self {
            operation: value,
            value_shape: value.value_shape(),
        }
    }
}
impl AdminDataFilter {
    #[must_use]
    pub const fn operation(&self) -> frontend_contract::FilterOperation {
        self.operation
    }
    #[must_use]
    pub const fn value_shape(&self) -> frontend_contract::FilterValueShape {
        self.value_shape
    }
    #[must_use]
    pub fn requires_value(&self) -> AdminBool {
        AdminBool::from(!matches!(
            self.value_shape,
            frontend_contract::FilterValueShape::None
        ))
    }
    #[must_use]
    pub fn requires_end(&self) -> AdminBool {
        AdminBool::from(matches!(
            self.value_shape,
            frontend_contract::FilterValueShape::Range
        ))
    }
}
#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminDataFilter>")]
#[schema(value_type = AdminOpenApiVec<AdminDataFilter, 100>)]
pub struct AdminDataFilters(AdminBoundedVec<AdminDataFilter>);
impl TryFrom<Vec<AdminDataFilter>> for AdminDataFilters {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminDataFilter>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataFilters {
    #[must_use]
    pub const fn as_slice(&self) -> &[AdminDataFilter] {
        self.0.as_slice()
    }
}
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminDataInputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}
impl From<frontend_contract::InputKind> for AdminDataInputKind {
    fn from(value: frontend_contract::InputKind) -> Self {
        match value {
            frontend_contract::InputKind::Checkbox => Self::Checkbox,
            frontend_contract::InputKind::Date => Self::Date,
            frontend_contract::InputKind::DateTime => Self::DateTime,
            frontend_contract::InputKind::Number => Self::Number,
            frontend_contract::InputKind::Text => Self::Text,
            frontend_contract::InputKind::Time => Self::Time,
            frontend_contract::InputKind::Uuid => Self::Uuid,
        }
    }
}
#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<AdminDataColumn>")]
#[schema(value_type = AdminOpenApiVec<AdminDataColumn, 10_000>)]
pub struct AdminDataColumns(AdminBoundedVec<AdminDataColumn>);
impl TryFrom<Vec<AdminDataColumn>> for AdminDataColumns {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<AdminDataColumn>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataColumns {
    #[must_use]
    pub const fn as_slice(&self) -> &[AdminDataColumn] {
        self.0.as_slice()
    }
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataRow {
    #[contract_struct_api(slice = AdminText)]
    values: AdminTexts,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataTableView {
    #[contract_struct_api(slice = AdminDataColumn)]
    columns: AdminDataColumns,
    #[contract_struct_api(slice = AdminDataRow)]
    items: AdminDataRows,
    #[contract_struct_api(copy_ref)]
    table: AdminDataTable,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: AdminPageTotal,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataTableCatalog {
    #[contract_struct_api(slice = AdminDataTable)]
    items: AdminDataTables,
}
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr, newtype::Display)]
#[bounded_string(
    max = 262_144usize,
    chars,
    serde,
    utoipa,
    description = "bounded administrator audit CSV export"
)]
pub struct AdminAuditExportCsv(String);
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditExport {
    #[schema(value_type = String, max_length = 262_144)]
    #[contract_struct_api(borrow)]
    csv: AdminAuditExportCsv,
}
#[derive(
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSettingsView {
    #[contract_struct_api(borrow)]
    default_admin_route: AdminDefaultRoute,
    #[contract_struct_api(option_borrow)]
    main_logo: Option<AdminMainLogo>,
    #[contract_struct_api(option_borrow)]
    organization_contacts: Option<AdminOrganizationContacts>,
    #[contract_struct_api(option_borrow)]
    organization_name: Option<AdminOrganizationName>,
    #[contract_struct_api(option_borrow)]
    primary_color: Option<AdminPrimaryColor>,
    #[contract_struct_api(borrow)]
    site_name: AdminSiteName,
    #[contract_struct_api(option_borrow)]
    support_url: Option<AdminSupportUrl>,
    #[contract_struct_api(option_borrow)]
    tab_title: Option<AdminTabTitle>,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminBrandingView {
    default_admin_route: AdminDefaultRoute,
    main_logo: Option<AdminMainLogo>,
    primary_color: Option<AdminPrimaryColor>,
    site_name: AdminSiteName,
    support_url: Option<AdminSupportUrl>,
    tab_title: Option<AdminTabTitle>,
}
impl AdminBrandingView {
    #[must_use]
    pub fn from_settings(value: &AdminSettingsView) -> Self {
        Self {
            default_admin_route: value.default_admin_route.clone(),
            main_logo: value.main_logo.clone(),
            primary_color: value.primary_color.clone(),
            site_name: value.site_name.clone(),
            support_url: value.support_url.clone(),
            tab_title: value.tab_title.clone(),
        }
    }
    #[must_use]
    pub const fn default_admin_route(&self) -> &AdminDefaultRoute {
        &self.default_admin_route
    }
    #[must_use]
    pub const fn main_logo(&self) -> Option<&AdminMainLogo> {
        self.main_logo.as_ref()
    }
    #[must_use]
    pub const fn primary_color(&self) -> Option<&AdminPrimaryColor> {
        self.primary_color.as_ref()
    }
    #[must_use]
    pub const fn site_name(&self) -> &AdminSiteName {
        &self.site_name
    }
    #[must_use]
    pub const fn support_url(&self) -> Option<&AdminSupportUrl> {
        self.support_url.as_ref()
    }
    #[must_use]
    pub const fn tab_title(&self) -> Option<&AdminTabTitle> {
        self.tab_title.as_ref()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateSettingsReq {
    #[schema(max_items = 6)]
    clear: AdminOptionalSettings,
    default_admin_route: Option<AdminDefaultRoute>,
    main_logo: Option<AdminMainLogo>,
    organization_contacts: Option<AdminOrganizationContacts>,
    organization_name: Option<AdminOrganizationName>,
    primary_color: Option<AdminPrimaryColor>,
    site_name: Option<AdminSiteName>,
    support_url: Option<AdminSupportUrl>,
    tab_title: Option<AdminTabTitle>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminSettingInputKind {
    Text,
    TextArea,
    Url,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::FromInner)]
pub struct AdminSettingLabel(&'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::FromInner)]
pub struct AdminSettingName(&'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminSettingSpec {
    input_kind: AdminSettingInputKind,
    label: AdminSettingLabel,
    name: AdminSettingName,
    optionality: AdminSettingOptionality,
}
impl AdminSettingSpec {
    #[must_use]
    pub const fn input_kind(self) -> AdminSettingInputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(self) -> AdminSettingLabel {
        self.label
    }
    #[must_use]
    pub const fn name(self) -> AdminSettingName {
        self.name
    }
    #[must_use]
    pub const fn optionality(self) -> AdminSettingOptionality {
        self.optionality
    }
    #[must_use]
    pub fn required(self) -> AdminBool {
        AdminBool::from(matches!(
            self.optionality,
            AdminSettingOptionality::Required
        ))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminSettingOptionality {
    Clearable(AdminOptionalSetting),
    Required,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract::UnitEnumCatalog,
    frontend_contract::UnitEnumIndex,
)]
pub enum AdminSetting {
    DefaultRoute,
    SiteName,
    TabTitle,
    OrganizationName,
    OrganizationContacts,
    SupportUrl,
    PrimaryColor,
    MainLogo,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    frontend_contract::UnitEnumCatalog,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminOptionalSetting {
    TabTitle,
    OrganizationName,
    OrganizationContacts,
    SupportUrl,
    PrimaryColor,
    MainLogo,
}
impl AdminSetting {
    #[must_use]
    pub fn spec(self) -> AdminSettingSpec {
        match self {
            Self::DefaultRoute => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from("Default route"),
                name: AdminSettingName::from("default_admin_route"),
                optionality: AdminSettingOptionality::Required,
            },
            Self::SiteName => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from("Site name"),
                name: AdminSettingName::from("site_name"),
                optionality: AdminSettingOptionality::Required,
            },
            Self::TabTitle => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from("Tab title"),
                name: AdminSettingName::from("tab_title"),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::TabTitle),
            },
            Self::OrganizationName => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from("Organization"),
                name: AdminSettingName::from("organization_name"),
                optionality: AdminSettingOptionality::Clearable(
                    AdminOptionalSetting::OrganizationName,
                ),
            },
            Self::OrganizationContacts => AdminSettingSpec {
                input_kind: AdminSettingInputKind::TextArea,
                label: AdminSettingLabel::from("Organization contacts"),
                name: AdminSettingName::from("organization_contacts"),
                optionality: AdminSettingOptionality::Clearable(
                    AdminOptionalSetting::OrganizationContacts,
                ),
            },
            Self::SupportUrl => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Url,
                label: AdminSettingLabel::from("Support URL"),
                name: AdminSettingName::from("support_url"),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::SupportUrl),
            },
            Self::PrimaryColor => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from("Primary color"),
                name: AdminSettingName::from("primary_color"),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::PrimaryColor),
            },
            Self::MainLogo => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Url,
                label: AdminSettingLabel::from("Main logo URL"),
                name: AdminSettingName::from("main_logo"),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::MainLogo),
            },
        }
    }
}
impl AdminUpdateSettingsReq {
    #[must_use]
    pub const fn new(
        default_admin_route: Option<AdminDefaultRoute>,
        main_logo: Option<AdminMainLogo>,
        organization_contacts: Option<AdminOrganizationContacts>,
        organization_name: Option<AdminOrganizationName>,
        primary_color: Option<AdminPrimaryColor>,
        site_name: Option<AdminSiteName>,
        support_url: Option<AdminSupportUrl>,
        tab_title: Option<AdminTabTitle>,
        clear: AdminOptionalSettings,
    ) -> Self {
        Self {
            clear,
            default_admin_route,
            main_logo,
            organization_contacts,
            organization_name,
            primary_color,
            site_name,
            support_url,
            tab_title,
        }
    }
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Option<AdminDefaultRoute>,
        Option<AdminMainLogo>,
        Option<AdminOrganizationContacts>,
        Option<AdminOrganizationName>,
        Option<AdminPrimaryColor>,
        Option<AdminSiteName>,
        Option<AdminSupportUrl>,
        Option<AdminTabTitle>,
        AdminOptionalSettings,
    ) {
        (
            self.default_admin_route,
            self.main_logo,
            self.organization_contacts,
            self.organization_name,
            self.primary_color,
            self.site_name,
            self.support_url,
            self.tab_title,
            self.clear,
        )
    }
    #[must_use]
    pub fn has_fields(&self) -> AdminBool {
        AdminBool::from(
            self.default_admin_route.is_some()
                || self.main_logo.is_some()
                || self.organization_contacts.is_some()
                || self.organization_name.is_some()
                || self.primary_color.is_some()
                || self.site_name.is_some()
                || self.support_url.is_some()
                || self.tab_title.is_some()
                || !self.clear.as_ref().is_empty(),
        )
    }
    #[must_use]
    pub fn is_valid(&self) -> AdminBool {
        let unique = self
            .clear
            .as_ref()
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        AdminBool::from(
            unique.len() == self.clear.as_ref().len()
                && self.clear.as_ref().len() <= AdminOptionalSetting::ALL.len()
                && !(self.main_logo.is_some() && unique.contains(&AdminOptionalSetting::MainLogo))
                && !(self.organization_contacts.is_some()
                    && unique.contains(&AdminOptionalSetting::OrganizationContacts))
                && !(self.organization_name.is_some()
                    && unique.contains(&AdminOptionalSetting::OrganizationName))
                && !(self.primary_color.is_some()
                    && unique.contains(&AdminOptionalSetting::PrimaryColor))
                && !(self.support_url.is_some()
                    && unique.contains(&AdminOptionalSetting::SupportUrl))
                && !(self.tab_title.is_some() && unique.contains(&AdminOptionalSetting::TabTitle)),
        )
    }
}
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct AdminNoBody;

#[derive(Clone, Debug, newtype::BoundedString, newtype::Display)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator session identifier"
)]
pub struct AdminSessionIdentifier(String);

#[derive(Clone, Debug, newtype::BoundedString, newtype::Display)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator session timestamp"
)]
pub struct AdminSessionTimestamp(String);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct AdminSessionView {
    created_at: AdminSessionTimestamp,
    expires_at: AdminSessionTimestamp,
    id: AdminSessionIdentifier,
    #[serde(default)]
    is_current: AdminBool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct AdminSessionsPage {
    items: AdminSessionViews,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
impl AdminSessionsPage {
    #[must_use]
    pub const fn new(items: AdminSessionViews, total: AdminPageTotal) -> Self {
        Self { items, total }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminSessionView] {
        self.items.0.as_slice()
    }
    #[must_use]
    pub const fn total(&self) -> AdminPageTotal {
        self.total
    }
}
fn admin_permission_requirement(
    permission: AdminPermission,
) -> frontend_contract::AuthenticationRequirement {
    frontend_contract::AuthenticationRequirement::Permission(frontend_contract::ContractStr::from(
        permission.as_str().get(),
    ))
}
impl AdminSessionView {
    #[must_use]
    pub const fn new(
        created_at: AdminSessionTimestamp,
        expires_at: AdminSessionTimestamp,
        id: AdminSessionIdentifier,
        is_current: AdminBool,
    ) -> Self {
        Self {
            created_at,
            expires_at,
            id,
            is_current,
        }
    }
    #[must_use]
    pub const fn created_at(&self) -> &AdminSessionTimestamp {
        &self.created_at
    }
    #[must_use]
    pub const fn expires_at(&self) -> &AdminSessionTimestamp {
        &self.expires_at
    }
    #[must_use]
    pub const fn id(&self) -> &AdminSessionIdentifier {
        &self.id
    }
    #[must_use]
    pub const fn is_current(&self) -> AdminBool {
        self.is_current
    }
}

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::RouteErrorPolicy::Authentication,
    openapi_operation_id = "sign_in",
    path = "/auth/sign_in",
    request = AdminSignInReq,
    request_body = frontend_contract::RouteRequestBody::Json,
    response = AdminSignInRes,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport,
)]
pub struct AdminSignInRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::RouteErrorPolicy::Authentication,
    openapi_operation_id = "refresh",
    path = "/auth/refresh",
    request = AdminNoBody,
    response = AdminSignInRes,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport,
)]
pub struct AdminRefreshRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Authenticated,
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
    error_policy = frontend_contract::RouteErrorPolicy::Default,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = AdminNoBody,
    response = AuthenticatedAdmin,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::AuthenticatedTransport,
)]
pub struct AdminMeRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "change_own_password", path = "/auth/password", request = AdminChangeOwnPasswordReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminChangeOwnPasswordRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sign_out", path = "/auth/sign_out", request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSignOutRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sessions", path = "/auth/sessions", request = AdminNoBody, response = AdminSessionsPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_session", path = "/auth/sessions/{session_id}", path_parameter = AdminSessionIdentifier, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeSessionRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::UsersRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = AdminNoBody, response = AdminUsersPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListUsersRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::UsersCreate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_user", path = "/users", request = AdminCreateUserReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminCreateUserRes, success_status = frontend_contract::SuccessStatus::Code201, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_user", path = "/users/{user_id}", path_parameter = AdminUserId, request = AdminUpdateUserReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Delete, authentication = admin_permission_requirement(AdminPermission::UsersDelete), method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_user", path = "/users/{user_id}", path_parameter = AdminUserId, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDeleteUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_password", path = "/users/{user_id}/password", path_parameter = AdminUserId, request = AdminSetUserPasswordReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserPasswordRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_ban", path = "/users/{user_id}/ban", path_parameter = AdminUserId, request = AdminSetUserBanReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserBanRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::UserRolesUpdate), method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_roles", path = "/users/{user_id}/roles", path_parameter = AdminUserId, request = AdminSetUserRolesReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserRolesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::RolesRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_roles", path = "/roles", request = AdminNoBody, response = AdminRolesPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListRolesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::RolesCreate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_role", path = "/roles", request = AdminCreateRoleReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminCreateRoleRes, success_status = frontend_contract::SuccessStatus::Code201, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::RolesUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_role", path = "/roles/{role_id}", path_parameter = AdminRoleId, request = AdminUpdateRoleReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Delete, authentication = admin_permission_requirement(AdminPermission::RolesDelete), method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_role", path = "/roles/{role_id}", path_parameter = AdminRoleId, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDeleteRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::RolePermissionsUpdate), method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_role_permissions", path = "/roles/{role_id}/permissions", path_parameter = AdminRoleId, request = AdminSetRolePermissionsReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetRolePermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::PermissionsRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_permissions", path = "/permissions", request = AdminNoBody, response = AdminPermissionsPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListPermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::ValidatedRead, authentication = admin_permission_requirement(AdminPermission::AuditLogRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "audit_log", path = "/audit_log", request = AdminNoBody, response = AdminAuditPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditLogRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::ValidatedRead, authentication = admin_permission_requirement(AdminPermission::AuditLogExport), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "export_audit_log", path = "/audit_log/export", request = AdminNoBody, response = AdminAuditExport, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditExportRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = frontend_contract::AuthenticationRequirement::Public, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "branding", path = "/branding", request = AdminNoBody, response = AdminBrandingView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::PublicTransport)]
pub struct AdminBrandingRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::TablesRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_data_tables", path = "/tables", request = AdminNoBody, response = AdminDataTableCatalog, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDataTablesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::ValidatedRead, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "read_data_table", path = "/tables/{table}", path_parameter = AdminDataTable, request = AdminNoBody, response = AdminDataTableView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDataTableRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::SystemSettingsRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "settings", path = "/system_settings", request = AdminNoBody, response = AdminSettingsView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSettingsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::Default, authentication = admin_permission_requirement(AdminPermission::SystemSettingsUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system_settings", request = AdminUpdateSettingsReq, request_body = frontend_contract::RouteRequestBody::Json, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateSettingsRoute;

#[derive(Clone, Copy, Debug, PartialEq, Eq, frontend_contract::RouteCatalog)]
#[route_catalog(
    family = AdminAuthenticationRouteFamily,
    body_limit = ADMIN_API_BODY_MAX_BYTES_VALUE,
)]
pub enum AdminRoute {
    #[route_catalog_route(AdminAuditLogRoute)]
    Audit,
    #[route_catalog_route(AdminAuditExportRoute)]
    AuditExport,
    #[route_catalog_route(AdminBrandingRoute)]
    Branding,
    #[route_catalog_route(AdminDataTableRoute)]
    DataTable(AdminDataTable),
    #[route_catalog_route(AdminDataTablesRoute)]
    DataTables,
    #[route_catalog_route(AdminChangeOwnPasswordRoute)]
    ChangeOwnPassword,
    #[route_catalog_route(AdminCreateRoleRoute)]
    CreateRole,
    #[route_catalog_route(AdminCreateUserRoute)]
    CreateUser,
    #[route_catalog_route(AdminDeleteRoleRoute)]
    DeleteRole(AdminRoleId),
    #[route_catalog_route(AdminDeleteUserRoute)]
    DeleteUser(AdminUserId),
    #[route_catalog_route(AdminMeRoute)]
    Me,
    #[route_catalog_route(
        contract = frontend_contract::RouteContract::new(
            admin_permission_requirement(AdminPermission::MetricsRead),
            frontend_contract::HttpMethod::Get,
            frontend_contract::MutationKind::ReadOnly,
            frontend_contract::ContractStr::from(str_constants::METRICS),
            frontend_contract::SuccessStatus::Code200,
        ),
        path = str_constants::METRICS,
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::RouteContract::new(
            admin_permission_requirement(AdminPermission::OpenApiRead),
            frontend_contract::HttpMethod::Get,
            frontend_contract::MutationKind::ReadOnly,
            frontend_contract::ContractStr::from(str_constants::OPENAPI_JSON),
            frontend_contract::SuccessStatus::Code200,
        ),
        path = str_constants::OPENAPI_JSON,
        exclude_from_family,
    )]
    OpenApi,
    #[route_catalog_route(AdminListPermissionsRoute)]
    Permissions,
    #[route_catalog_route(AdminRefreshRoute)]
    Refresh,
    #[route_catalog_route(AdminRevokeAllSessionsRoute)]
    RevokeAllSessions,
    #[route_catalog_route(AdminRevokeSessionRoute)]
    RevokeSession,
    #[route_catalog_route(AdminListRolesRoute)]
    Roles,
    #[route_catalog_route(AdminSetRolePermissionsRoute)]
    SetRolePermissions(AdminRoleId),
    #[route_catalog_route(AdminSetUserBanRoute)]
    SetUserBan(AdminUserId),
    #[route_catalog_route(AdminSetUserPasswordRoute)]
    SetUserPassword(AdminUserId),
    #[route_catalog_route(AdminSetUserRolesRoute)]
    SetUserRoles(AdminUserId),
    #[route_catalog_route(AdminSettingsRoute)]
    Settings,
    #[route_catalog_route(AdminSignInRoute)]
    SignIn,
    #[route_catalog_route(AdminSignOutRoute)]
    SignOut,
    #[route_catalog_route(AdminSessionsRoute)]
    Sessions,
    #[route_catalog_route(AdminUpdateRoleRoute)]
    UpdateRole(AdminRoleId),
    #[route_catalog_route(AdminUpdateSettingsRoute)]
    UpdateSettings,
    #[route_catalog_route(AdminUpdateUserRoute)]
    UpdateUser(AdminUserId),
    #[route_catalog_route(AdminListUsersRoute)]
    Users,
    #[route_catalog_route(
        contract = frontend_contract::RouteContract::new(
            frontend_contract::AuthenticationRequirement::Public,
            frontend_contract::HttpMethod::Get,
            frontend_contract::MutationKind::ReadOnly,
            frontend_contract::ContractStr::from(str_constants::COMMON_ROUTES_GIT_INFO),
            frontend_contract::SuccessStatus::Code200,
        ),
        path = str_constants::API_V1_GIT_INFO,
        exclude_from_family,
    )]
    Version,
}
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::Display)]
pub struct AdminDataTableFrontendPath(Box<str>);
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::AsRefStr, newtype::Display)]
pub struct AdminRoutePath(Box<str>);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminRoutePathError {
    TooLong,
}
impl std::fmt::Display for AdminRoutePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong => f.write_str(str_constants::ADMINISTRATOR_ROUTE_PATH_IS_TOO_LONG),
        }
    }
}
impl TryFrom<String> for AdminRoutePath {
    type Error = AdminRoutePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 8192usize {
            Err(AdminRoutePathError::TooLong)
        } else {
            Ok(Self(value.into_boxed_str()))
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
pub struct AdminPagePathRef<'path_lt>(&'path_lt str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::IntoStaticStr)]
pub enum AdminFrontendPath {
    #[strum(serialize = "/admin/assets")]
    Assets,
    #[strum(serialize = "/admin/metrics")]
    Metrics,
    #[strum(serialize = "/admin/openapi.json")]
    OpenApiDocument,
    #[strum(serialize = "/admin/swagger_ui")]
    OpenApi,
    #[strum(serialize = "/admin/permissions")]
    Permissions,
    #[strum(serialize = "/admin/profile")]
    Profile,
    #[strum(serialize = "/admin/roles")]
    Roles,
    #[strum(serialize = "/admin/sessions")]
    Sessions,
    #[strum(serialize = "/admin")]
    Root,
    #[strum(serialize = "/admin/sign_in")]
    SignIn,
    #[strum(serialize = "/admin/settings")]
    Settings,
    #[strum(serialize = "/admin/{table}")]
    Tables,
    #[strum(serialize = "/admin/users")]
    Users,
    #[strum(serialize = "/admin/version")]
    Version,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::IntoStaticStr,
    frontend_contract::UnitEnumCatalog,
)]
pub enum AdminHtmlAction {
    #[strum(serialize = "/admin/actions/profile/password")]
    ProfilePassword,
    #[strum(serialize = "/admin/actions/roles/create")]
    RoleCreate,
    #[strum(serialize = "/admin/actions/roles/delete")]
    RoleDelete,
    #[strum(serialize = "/admin/actions/roles/permissions")]
    RolePermissions,
    #[strum(serialize = "/admin/actions/roles/update")]
    RoleUpdate,
    #[strum(serialize = "/admin/actions/sessions/revoke")]
    SessionRevoke,
    #[strum(serialize = "/admin/actions/settings/update")]
    SettingsUpdate,
    #[strum(serialize = "/admin/actions/sign_in")]
    SignIn,
    #[strum(serialize = "/admin/actions/sign_out")]
    SignOut,
    #[strum(serialize = "/admin/actions/users/ban")]
    UserBan,
    #[strum(serialize = "/admin/actions/users/create")]
    UserCreate,
    #[strum(serialize = "/admin/actions/users/delete")]
    UserDelete,
    #[strum(serialize = "/admin/actions/users/password")]
    UserPassword,
    #[strum(serialize = "/admin/actions/users/roles")]
    UserRoles,
    #[strum(serialize = "/admin/actions/users/update")]
    UserUpdate,
}
impl AdminHtmlAction {
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
    #[must_use]
    pub fn route_name(self) -> frontend_contract::ContractStr {
        admin_path_route_name(AdminPagePathRef::from(self.get()))
    }
}
impl frontend_contract::HandlerContract for AdminHtmlAction {
    fn method(self) -> frontend_contract::RouteMethod {
        frontend_contract::RouteMethod::Post
    }
    fn path(self) -> frontend_contract::HandlerPath {
        frontend_contract::HandlerPath::from(self.get())
    }
}
impl AdminFrontendPath {
    pub fn all_pages() -> impl Iterator<Item = Self> {
        [Self::Root, Self::SignIn]
            .into_iter()
            .chain(AdminPage::specs().iter().map(|spec| spec.frontend_path()))
    }
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
}
impl frontend_contract::HandlerContract for AdminFrontendPath {
    fn method(self) -> frontend_contract::RouteMethod {
        frontend_contract::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::HandlerPath {
        frontend_contract::HandlerPath::from(self.get())
    }
}
impl From<AdminDataTable> for AdminDataTableFrontendPath {
    fn from(value: AdminDataTable) -> Self {
        Self(format!("{}/{}", AdminFrontendPath::Root.get(), value).into_boxed_str())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, frontend_contract::PageCatalog)]
#[page_catalog(
    spec = AdminPageSpec,
    path_ref = AdminPagePathRef,
    inventory = ADMIN_PAGE_SPECS,
)]
pub enum AdminPage {
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::CsrTableQuery, None),
        path = AdminFrontendPath::Users,
        route = AdminRoute::Users,
        title = AdminPageTitle::Users,
    )]
    Users,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::CsrTableQuery, None),
        path = AdminFrontendPath::Roles,
        route = AdminRoute::Roles,
        title = AdminPageTitle::Roles,
    )]
    Roles,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::CsrTableQuery, None),
        path = AdminFrontendPath::Permissions,
        route = AdminRoute::Permissions,
        title = AdminPageTitle::Permissions,
    )]
    Permissions,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Csr,
            Some(AdminPageNavigation::Settings),
        ),
        path = AdminFrontendPath::Settings,
        route = AdminRoute::Settings,
        title = AdminPageTitle::Settings,
    )]
    Settings,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(AdminPageClientMode::Csr, None),
        path = AdminFrontendPath::Tables,
        route = AdminRoute::DataTables,
        title = AdminPageTitle::Tables,
    )]
    Tables,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Csr,
            Some(AdminPageNavigation::Sessions),
        ),
        path = AdminFrontendPath::Sessions,
        route = AdminRoute::Sessions,
        title = AdminPageTitle::Sessions,
    )]
    Sessions,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Ssr,
            Some(AdminPageNavigation::Metrics),
        ),
        path = AdminFrontendPath::Metrics,
        route = AdminRoute::Metrics,
        title = AdminPageTitle::Metrics,
    )]
    Metrics,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Ssr,
            Some(AdminPageNavigation::Version),
        ),
        path = AdminFrontendPath::Version,
        route = AdminRoute::Version,
        title = AdminPageTitle::Version,
    )]
    Version,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Csr,
            Some(AdminPageNavigation::Profile),
        ),
        path = AdminFrontendPath::Profile,
        route = AdminRoute::ChangeOwnPassword,
        title = AdminPageTitle::Profile,
    )]
    Profile,
    #[page_catalog_page(
        capability = AdminPageCapability::Swagger,
        metadata = AdminPageMetadata::new(
            AdminPageClientMode::Ssr,
            Some(AdminPageNavigation::OpenApi),
        ),
        path = AdminFrontendPath::OpenApi,
        route = AdminRoute::OpenApi,
        title = AdminPageTitle::Api,
    )]
    OpenApi,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageCapability {
    Always,
    Swagger,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageClientMode {
    Csr,
    CsrTableQuery,
    Ssr,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdminPageNavigation {
    OpenApi,
    Metrics,
    Profile,
    Sessions,
    Settings,
    Version,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageMetadata {
    client_mode: AdminPageClientMode,
    navigation: Option<AdminPageNavigation>,
}
impl AdminPageMetadata {
    const fn new(
        client_mode: AdminPageClientMode,
        navigation: Option<AdminPageNavigation>,
    ) -> Self {
        Self {
            client_mode,
            navigation,
        }
    }
}
impl AdminPageClientMode {
    fn supports_csr(self) -> AdminBool {
        AdminBool::from(matches!(self, Self::Csr | Self::CsrTableQuery))
    }
    fn uses_table_query(self) -> AdminBool {
        AdminBool::from(matches!(self, Self::CsrTableQuery))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AdminPageTitle {
    Api,
    Metrics,
    Permissions,
    Profile,
    Roles,
    Sessions,
    Settings,
    Tables,
    Users,
    Version,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageSpec {
    capability: AdminPageCapability,
    metadata: AdminPageMetadata,
    page: AdminPage,
    path: AdminFrontendPath,
    route: AdminRoute,
    title: AdminPageTitle,
}
impl AdminPageSpec {
    const fn new(
        capability: AdminPageCapability,
        metadata: AdminPageMetadata,
        page: AdminPage,
        path: AdminFrontendPath,
        route: AdminRoute,
        title: AdminPageTitle,
    ) -> Self {
        Self {
            capability,
            metadata,
            page,
            path,
            route,
            title,
        }
    }
    #[must_use]
    pub const fn capability(self) -> AdminPageCapability {
        self.capability
    }
    #[must_use]
    pub const fn client_mode(self) -> AdminPageClientMode {
        self.metadata.client_mode
    }
    #[must_use]
    pub const fn navigation(self) -> Option<AdminPageNavigation> {
        self.metadata.navigation
    }
    #[must_use]
    pub const fn frontend_path(self) -> AdminFrontendPath {
        self.path
    }
    #[must_use]
    pub const fn page(self) -> AdminPage {
        self.page
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(self.path.get())
    }
    #[must_use]
    pub fn route_name(self) -> frontend_contract::ContractStr {
        admin_path_route_name(AdminPagePathRef::from(self.path.get()))
    }
    #[must_use]
    pub const fn route(self) -> AdminRoute {
        self.route
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(match self.title {
            AdminPageTitle::Api => str_constants::API_ALT,
            AdminPageTitle::Metrics => str_constants::METRICS_ALT,
            AdminPageTitle::Permissions => str_constants::PERMISSIONS,
            AdminPageTitle::Profile => str_constants::PROFILE,
            AdminPageTitle::Roles => str_constants::ROLES,
            AdminPageTitle::Sessions => str_constants::SESSIONS_ALT,
            AdminPageTitle::Settings => str_constants::SETTINGS,
            AdminPageTitle::Tables => str_constants::TABLES,
            AdminPageTitle::Users => str_constants::USERS,
            AdminPageTitle::Version => str_constants::VERSION_ALT,
        })
    }
}
fn admin_path_route_name(path: AdminPagePathRef<'static>) -> frontend_contract::ContractStr {
    frontend_contract::ContractStr::from(
        path.0
            .rsplit_once('/')
            .map_or(path.0, |(_prefix, name)| name),
    )
}
impl AdminPage {
    pub fn navigation() -> impl Iterator<Item = Self> {
        let mut pages = Self::specs()
            .iter()
            .filter_map(|spec| {
                spec.navigation()
                    .map(|navigation| (navigation, spec.page()))
            })
            .collect::<Vec<_>>();
        pages.sort_by_key(|(navigation, _page)| *navigation);
        pages.into_iter().map(|(_navigation, page)| page)
    }

    #[must_use]
    pub fn supports_csr(self) -> AdminBool {
        self.spec().client_mode().supports_csr()
    }
    #[must_use]
    pub fn uses_table_query(self) -> AdminBool {
        self.spec().client_mode().uses_table_query()
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::ContractStr {
        self.spec().path()
    }
    #[must_use]
    pub const fn route(self) -> Option<AdminRoute> {
        Some(self.spec().route())
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::ContractStr {
        self.spec().title()
    }
    #[must_use]
    pub fn authentication(self) -> frontend_contract::AuthenticationRequirement {
        self.spec().route().contract().authentication()
    }
}
impl AdminRoute {
    #[must_use]
    pub fn path(self) -> AdminRoutePath {
        let suffix = self.catalog_path();
        if matches!(self, Self::Version) {
            AdminRoutePath::try_from(String::from(suffix)).unwrap_or_default()
        } else {
            admin_api_route_path(suffix)
        }
    }
}
#[must_use]
pub fn admin_parameterized_route_path<Route>(parameter: &Route::Parameter) -> AdminRoutePath
where
    Route: frontend_contract::ParameterizedRoute,
{
    admin_api_route_path(frontend_contract::typed_parameterized_route_path::<Route>(
        parameter,
    ))
}
fn admin_api_route_path(suffix: frontend_contract::ParameterizedRoutePath) -> AdminRoutePath {
    AdminRoutePath::try_from(format!(
        "{}{}{suffix}",
        str_constants::API_V1,
        AdminFrontendPath::Root.get(),
        suffix = String::from(suffix),
    ))
    .unwrap_or_default()
}
#[cfg(test)]
mod tests {
    fn assert_rejects_unknown_field<Value>(json: &str)
    where
        Value: serde::de::DeserializeOwned,
    {
        let Err(_error) = serde_json::from_str::<Value>(json) else {
            panic!("30bbf690");
        };
    }
    #[test]
    fn change_own_password_request_has_no_session_revocation_choice() {
        let request = super::AdminChangeOwnPasswordReq::new(
            super::AdminPassword::try_from(String::from("Current-password1")).expect("c10e4db7"),
            super::AdminNewPassword::try_from(String::from("New-password2")).expect("5932a1fe"),
        );
        let json = serde_json::to_value(request).expect("06ba3ef9");
        assert_eq!(
            json,
            serde_json::json!({
                "current_password": "Current-password1",
                "new_password": "New-password2",
            })
        );
        assert_rejects_unknown_field::<super::AdminChangeOwnPasswordReq>(
            r#"{"current_password":"Current-password1","new_password":"New-password2","revoke_other_sessions":false}"#,
        );
    }
    #[test]
    fn administrator_collections_enforce_item_limit_for_construction_and_deserialization() {
        let maximum_values = vec![
            super::AdminRoleId::try_from(1i64).expect("4cd8c4ef");
            super::ADMIN_COLLECTION_MAX_ITEMS
        ];
        let Ok(maximum_role_ids) = super::AdminRoleIds::try_from(maximum_values) else {
            panic!("bce86c7b");
        };
        assert_eq!(
            maximum_role_ids.as_ref().len(),
            super::ADMIN_COLLECTION_MAX_ITEMS
        );
        let oversized = vec![
            super::AdminRoleId::try_from(1i64).expect("1c1b920f");
            super::ADMIN_COLLECTION_MAX_ITEMS.saturating_add(1usize)
        ];
        assert!(matches!(
            super::AdminRoleIds::try_from(oversized),
            Err(super::AdminCollectionError::TooLong)
        ));
        let json = serde_json::json!(vec![
            1i64;
            super::ADMIN_COLLECTION_MAX_ITEMS.saturating_add(1usize)
        ])
        .to_string();
        let Err(_error) = serde_json::from_str::<super::AdminRoleIds>(&json) else {
            panic!("742a0bdd");
        };
    }
    #[test]
    fn authenticated_admin_checks_permissions_and_page_access() {
        let admin = super::AuthenticatedAdmin::new(
            super::AdminDisplayName::try_from(str_constants::ADMIN.to_owned()).expect("67f10787"),
            super::AdminUserId::try_from(1i64).expect("da64f9f1"),
            super::AdminLogin::try_from(str_constants::ROOT.to_owned()).expect("ced445ee"),
            super::AdminPermissionValues::try_from(vec![
                super::AdminPermissionValue::try_from(
                    super::AdminPermission::UsersRead.as_str().get().to_owned(),
                )
                .expect("837c99bb"),
            ])
            .expect("34462164"),
            super::AdminRoleNames::try_from(Vec::new()).expect("cc22bb17"),
        );
        assert!(bool::from(
            admin.has_permission(super::AdminPermission::UsersRead)
        ));
        assert!(!bool::from(
            admin.has_permission(super::AdminPermission::UsersUpdate)
        ));
        assert!(bool::from(admin.can_access(super::AdminPage::Users)));
        assert!(!bool::from(admin.can_access(super::AdminPage::Roles)));
        assert!(bool::from(admin.can_access(super::AdminPage::Profile)));
    }
    #[test]
    fn authentication_route_family_has_valid_coverage() {
        let descriptors = <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
        assert_eq!(descriptors.as_ref().len(), 28usize);
        assert_eq!(
            frontend_contract::validate_route_coverage(descriptors.as_ref()),
            Ok(())
        );
        assert_eq!(
            <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
                .map(frontend_contract::RouteBodyLimit::get),
            Some(super::admin_api_body_max_bytes().get())
        );
    }
    #[test]
    fn system_setting_types_match_database_constraints() {
        let Err(_empty_site_name_error) = super::AdminSiteName::try_from(String::new()) else {
            panic!("4cfb6820");
        };
        let Err(_blank_site_name_error) =
            super::AdminSiteName::try_from(str_constants::SPACE.to_owned())
        else {
            panic!("b5fba19e");
        };
        let _site_name =
            super::AdminSiteName::try_from(str_constants::ADMIN.to_owned()).expect("adb58327");
        let _default_route =
            super::AdminDefaultRoute::try_from(super::AdminFrontendPath::Users.get().to_owned())
                .expect("3582a0ec");
        let _table_default_route = super::AdminDefaultRoute::try_from(
            super::AdminDataTable::RolePermissions
                .frontend_path()
                .to_string(),
        )
        .expect("e3d42017");
        let Err(_invalid_route_error) =
            super::AdminDefaultRoute::try_from(str_constants::ROUTE.to_owned())
        else {
            panic!("bb0d454a");
        };
    }
    #[test]
    fn update_settings_reports_whether_it_contains_a_field() {
        let empty = super::AdminUpdateSettingsReq::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            super::AdminOptionalSettings::try_from(Vec::new()).expect("c4a1e2d3"),
        );
        assert!(!bool::from(empty.has_fields()));
        let with_site_name = super::AdminUpdateSettingsReq::new(
            None,
            None,
            None,
            None,
            None,
            Some(
                super::AdminSiteName::try_from(str_constants::ADMIN.to_owned()).expect("5db76a91"),
            ),
            None,
            None,
            super::AdminOptionalSettings::try_from(Vec::new()).expect("32e4e74d"),
        );
        assert!(bool::from(with_site_name.has_fields()));
        assert!(bool::from(with_site_name.is_valid()));
        let clear_logo = super::AdminUpdateSettingsReq::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            super::AdminOptionalSettings::try_from(vec![super::AdminOptionalSetting::MainLogo])
                .expect("96e94562"),
        );
        assert!(bool::from(clear_logo.has_fields()));
        assert!(bool::from(clear_logo.is_valid()));
    }

    #[test]
    fn setting_catalog_covers_read_and_update_wire_fields() {
        let empty_clear = super::AdminOptionalSettings::try_from(Vec::new()).expect("7f3a9c2e");
        let update = super::AdminUpdateSettingsReq::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            empty_clear,
        );
        let update_fields = serde_json::to_value(update)
            .expect("c84d1e6a")
            .as_object()
            .expect("49b2e7c1")
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let setting_fields = super::AdminSetting::ALL
            .into_iter()
            .map(|setting| setting.spec().name().as_ref().to_owned())
            .collect::<std::collections::BTreeSet<_>>();
        let mut expected_update_fields = setting_fields.clone();
        let _inserted = expected_update_fields.insert(String::from("clear"));
        assert_eq!(update_fields, expected_update_fields);

        let view = super::AdminSettingsView::new(
            super::AdminDefaultRoute::try_from(String::from("/admin/users")).expect("b6831fd4"),
            None,
            None,
            None,
            None,
            super::AdminSiteName::try_from(String::from("Admin")).expect("e15c7a93"),
            None,
            None,
        );
        let view_fields = serde_json::to_value(view)
            .expect("86d4a2f9")
            .as_object()
            .expect("21c9e5b7")
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(view_fields, setting_fields);
    }
    #[test]
    fn request_payloads_reject_unknown_fields() {
        assert_rejects_unknown_field::<super::AdminSignInReq>(
            str_constants::LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminCreateUserReq>(
            str_constants::DISPLAY_NAME_ADMIN_LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminUpdateUserReq>(
            str_constants::DISPLAY_NAME_ADMIN_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminSetUserPasswordReq>(
            str_constants::PASSWORD_SECRET_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminSetUserBanReq>(
            str_constants::IS_BANNED_TRUE_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminCreateRoleReq>(
            str_constants::NAME_ADMINISTRATOR_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminUpdateRoleReq>(
            str_constants::NAME_ADMINISTRATOR_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminSetUserRolesReq>(
            str_constants::ROLE_IDS_1_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminSetRolePermissionsReq>(
            str_constants::PERMISSION_IDS_1_UNKNOWN_TRUE,
        );
        assert_rejects_unknown_field::<super::AdminUpdateSettingsReq>(
            str_constants::SITE_NAME_ADMIN_UNKNOWN_TRUE,
        );
    }
    #[test]
    fn route_contract_keeps_custom_action_policy_and_path_together() {
        let route =
            super::AdminRoute::SetUserBan(super::AdminUserId::try_from(7).expect("8bed843c"));
        assert_eq!(route.path().as_ref(), "/api/v1/admin/users/7/ban");
        assert_eq!(
            route.contract().method(),
            frontend_contract::HttpMethod::Post
        );
        assert_eq!(
            route.contract().mutation(),
            frontend_contract::MutationKind::Mutating
        );
        assert_eq!(
            route.contract().authentication(),
            frontend_contract::AuthenticationRequirement::Permission(
                frontend_contract::ContractStr::from(
                    super::AdminPermission::UsersUpdate.as_str().get(),
                )
            )
        );
    }
    #[test]
    fn parameterized_admin_route_path_uses_typed_route_metadata() {
        let session_id = super::AdminSessionIdentifier::try_from(String::from("test-session"))
            .expect("84d51132");
        let path =
            super::admin_parameterized_route_path::<super::AdminRevokeSessionRoute>(&session_id);
        assert_eq!(path.as_ref(), "/api/v1/admin/auth/sessions/test-session");
    }
    #[test]
    fn html_action_inventory_has_unique_paths() {
        let paths = super::AdminHtmlAction::ALL
            .into_iter()
            .map(super::AdminHtmlAction::get)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(paths.len(), super::AdminHtmlAction::ALL.len());
    }
    #[test]
    fn open_api_page_uses_the_typed_authenticated_api_route() {
        let route = super::AdminRoute::OpenApi;
        assert_eq!(route.path().as_ref(), "/api/v1/admin/openapi.json");
        assert_eq!(
            route.contract().authentication(),
            frontend_contract::AuthenticationRequirement::Permission(
                frontend_contract::ContractStr::from(
                    super::AdminPermission::OpenApiRead.as_str().get(),
                ),
            )
        );
        assert_eq!(
            super::AdminPage::OpenApi.route(),
            Some(super::AdminRoute::OpenApi)
        );
        assert_eq!(
            super::AdminPage::OpenApi.spec().capability(),
            super::AdminPageCapability::Swagger
        );
        assert!(super::AdminPage::all().all(|page| {
            page == super::AdminPage::OpenApi
                || page.spec().capability() == super::AdminPageCapability::Always
        }));
    }
    #[test]
    fn removed_audit_log_page_is_not_a_frontend_route() {
        assert_eq!(
            super::AdminPage::from_path(super::AdminPagePathRef::from("/admin/audit-log")),
            None
        );
        let Err(_error) = super::AdminDefaultRoute::try_from(String::from("/admin/audit-log"))
        else {
            panic!("61f0ab3e");
        };
    }
    #[test]
    fn administrator_routes_use_snake_case_segments() {
        let frontend_paths = super::AdminFrontendPath::all_pages()
            .map(super::AdminFrontendPath::get)
            .collect::<Vec<_>>();
        assert_eq!(
            frontend_paths.len(),
            super::AdminPage::specs().len().saturating_add(2usize)
        );
        assert_eq!(
            frontend_paths
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            frontend_paths.len()
        );
        assert!(
            [
                super::AdminRoute::Audit,
                super::AdminRoute::AuditExport,
                super::AdminRoute::Settings,
                super::AdminRoute::SignIn,
                super::AdminRoute::SignOut,
            ]
            .iter()
            .all(|route| !route.path().as_ref().contains('-'))
        );
        assert!(frontend_paths.iter().all(|path| !path.contains('-')));
        assert!(
            super::AdminHtmlAction::ALL
                .iter()
                .all(|action| !action.get().contains('-'))
        );
    }
    #[test]
    fn password_debug_is_redacted() {
        let password =
            super::AdminPassword::try_from(String::from(str_constants::SECRET)).expect("9f3f5164");
        assert!(!format!("{password:?}").contains("secret"));
    }
    #[test]
    fn sign_in_accepts_only_login_and_password() {
        let basic = serde_json::json!({
            "login": "admin",
            "password": "correct_password"
        });
        let Ok(_basic_request) = serde_json::from_value::<super::AdminSignInReq>(basic) else {
            panic!("af47412d");
        };
        let legacy_mfa = serde_json::json!({
            "login": "admin",
            "mfa_proof": { "kind": "totp", "value": "123456" },
            "password": "correct_password"
        });
        let Err(_legacy_mfa_error) = serde_json::from_value::<super::AdminSignInReq>(legacy_mfa)
        else {
            panic!("89071e97");
        };
    }
    #[test]
    fn new_password_uses_the_shared_password_policy() {
        let _password =
            super::AdminNewPassword::try_from(str_constants::TEST_STRONG_PASSWORD.to_owned())
                .expect("da19950b");
        let Err(_weak_password_error) =
            super::AdminNewPassword::try_from(str_constants::PASSWORD.to_owned())
        else {
            panic!("24900f2f");
        };
    }
    #[test]
    fn admin_domain_values_follow_database_compatible_policies() {
        let _valid_login =
            super::AdminLogin::try_from(str_constants::ADMIN_USER_1.to_owned()).expect("e1cddebc");
        let Err(_reserved_login_error) =
            super::AdminLogin::try_from(str_constants::ADMIN.to_owned())
        else {
            panic!("ab23c76e");
        };
        let Err(_short_login_error) = super::AdminLogin::try_from(str_constants::AB.to_owned())
        else {
            panic!("ce5b9e72");
        };
        let _valid_display_name =
            super::AdminDisplayName::try_from(str_constants::ADMIN.to_owned()).expect("d315b74f");
        let Err(_blank_display_name_error) =
            super::AdminDisplayName::try_from(str_constants::SPACE.to_owned())
        else {
            panic!("1ccd43aa");
        };
        let _valid_role_name =
            super::AdminRoleName::try_from(str_constants::ADMIN_ALT.to_owned()).expect("713890e9");
        let Err(_reserved_role_name_error) =
            super::AdminRoleName::try_from(str_constants::ADMIN.to_owned())
        else {
            panic!("147fe35a");
        };
    }
    #[test]
    fn audit_details_enforce_serialized_byte_limit() {
        let accepted = super::SerdeJsonAdminAuditDetails::try_from(serde_json::json!({
            "operation": "create"
        }));
        let _accepted = accepted.expect("20697dc1");
        let oversized = super::SerdeJsonAdminAuditDetails::try_from(serde_json::Value::String(
            str_constants::A_ALT.repeat(super::ADMIN_AUDIT_DETAILS_MAX_BYTES),
        ));
        assert_eq!(
            oversized.err(),
            Some(super::AdminAuditDetailsTooLarge(
                super::AdminAuditDetailsBytes::from(
                    super::ADMIN_AUDIT_DETAILS_MAX_BYTES.saturating_add(2usize),
                ),
            ))
        );
    }
    #[test]
    fn table_sort_fields_reject_unknown_and_wrong_table_keys() {
        assert_eq!(
            super::AdminTableSortField::try_from_key(
                &super::AdminTableSortField::USER,
                super::AdminTableSortKeyRef::from(str_constants::LOGIN),
            ),
            Ok(super::AdminTableSortField::UserLogin)
        );
        assert_eq!(
            super::AdminTableSortField::try_from_key(
                &super::AdminTableSortField::USER,
                super::AdminTableSortKeyRef::from(str_constants::CREATED_AT),
            ),
            Err(super::AdminTableSortFieldTryFromKeyError)
        );
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository source policy requires iterator methods"
    )]
    fn data_tables_round_trip_and_require_read_permissions() {
        assert_eq!(super::AdminDataTable::ALL.len(), 12usize);
        assert_eq!(
            super::AdminDataTable::PG_ORDER,
            [
                super::AdminDataTable::Users,
                super::AdminDataTable::Roles,
                super::AdminDataTable::Permissions,
                super::AdminDataTable::UserRoles,
                super::AdminDataTable::RolePermissions,
                super::AdminDataTable::RefreshTokens,
                super::AdminDataTable::AccessSessions,
                super::AdminDataTable::LoginAttempts,
                super::AdminDataTable::AuditLog,
                super::AdminDataTable::SystemSettings,
                super::AdminDataTable::RateLimits,
                super::AdminDataTable::CleanupStatus,
            ]
        );
        assert_eq!(
            super::AdminPage::navigation().collect::<Vec<_>>(),
            vec![
                super::AdminPage::OpenApi,
                super::AdminPage::Metrics,
                super::AdminPage::Profile,
                super::AdminPage::Sessions,
                super::AdminPage::Settings,
                super::AdminPage::Version,
            ]
        );
        assert_eq!(
            super::AdminPage::all()
                .filter(|page| bool::from(page.supports_csr()))
                .collect::<Vec<_>>(),
            vec![
                super::AdminPage::Users,
                super::AdminPage::Roles,
                super::AdminPage::Permissions,
                super::AdminPage::Settings,
                super::AdminPage::Tables,
                super::AdminPage::Sessions,
                super::AdminPage::Profile,
            ]
        );
        assert_eq!(
            super::AdminPage::all()
                .filter(|page| bool::from(page.uses_table_query()))
                .collect::<Vec<_>>(),
            vec![
                super::AdminPage::Users,
                super::AdminPage::Roles,
                super::AdminPage::Permissions,
            ]
        );
        assert_eq!(
            super::AdminPage::navigation()
                .map(|page| page.spec().route_name().to_string())
                .collect::<Vec<_>>(),
            vec![
                String::from("swagger_ui"),
                String::from("metrics"),
                String::from("profile"),
                String::from("sessions"),
                String::from("settings"),
                String::from("version"),
            ]
        );
        assert_eq!(
            super::AdminHtmlAction::SignOut.route_name().as_ref(),
            "sign_out"
        );
        assert_eq!(
            frontend_contract::HandlerContract::method(super::AdminHtmlAction::SignOut),
            frontend_contract::RouteMethod::Post
        );
        assert_eq!(
            frontend_contract::HandlerContract::path(super::AdminHtmlAction::SignOut).get(),
            super::AdminHtmlAction::SignOut.get()
        );
        assert_eq!(
            frontend_contract::HandlerContract::method(super::AdminFrontendPath::Settings),
            frontend_contract::RouteMethod::Get
        );
        assert_eq!(
            frontend_contract::HandlerContract::path(super::AdminFrontendPath::Settings).get(),
            super::AdminFrontendPath::Settings.get()
        );
        assert!(super::AdminPage::navigation().all(|page| {
            let page_label = page.spec().route_name();
            super::AdminDataTable::PG_ORDER
                .iter()
                .all(|table| table.to_string() != page_label.as_ref())
        }));
        assert_eq!(
            super::AdminDataTable::ALL
                .into_iter()
                .filter(|table| bool::from(table.supports_filters()))
                .collect::<Vec<_>>(),
            vec![super::AdminDataTable::RolePermissions]
        );
        assert_eq!(
            super::AdminDataTable::PG_ORDER.map(|table| table.frontend_path().to_string()),
            [
                String::from("/admin/users"),
                String::from("/admin/roles"),
                String::from("/admin/permissions"),
                String::from("/admin/user_roles"),
                String::from("/admin/role_permissions"),
                String::from("/admin/refresh_tokens"),
                String::from("/admin/access_sessions"),
                String::from("/admin/login_attempts"),
                String::from("/admin/audit_log"),
                String::from("/admin/system_settings"),
                String::from("/admin/rate_limits"),
                String::from("/admin/cleanup_status"),
            ]
        );
        super::AdminDataTable::ALL.into_iter().for_each(|table| {
            let spec = table.spec();
            let columns = spec.columns().get().split(',').collect::<Vec<_>>();
            assert!(!columns.is_empty());
            assert_eq!(
                columns
                    .iter()
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                columns.len()
            );
            assert!(!spec.order().get().is_empty());
            assert_eq!(spec.permission(), table.permission());
            assert_eq!(spec.supports_filters(), table.supports_filters());
            assert_eq!(
                super::AdminDataTable::try_from(table.to_string()).expect("0596134b"),
                table
            );
            assert_eq!(
                super::AdminDataTable::from_frontend_path(super::AdminPagePathRef::from(
                    table.frontend_path().as_ref(),
                )),
                Some(table)
            );
            assert!(table.permission().as_str().get().ends_with(":read"));
        });
        assert_eq!(
            super::AdminDataTable::from_frontend_path(super::AdminPagePathRef::from(
                "/admin/tables",
            )),
            None
        );
        assert_eq!(
            super::AdminDataTable::from_frontend_path(super::AdminPagePathRef::from(
                "/admin/profile",
            )),
            None
        );
    }

    #[test]
    fn page_limit_rejects_zero_and_values_above_server_maximum() {
        let Err(_zero_error) =
            serde_json::from_str::<super::AdminPageLimit>(str_constants::VALUE_0)
        else {
            panic!("e8fd3a29");
        };
        let Err(_above_maximum_error) =
            serde_json::from_str::<super::AdminPageLimit>(str_constants::VALUE_101)
        else {
            panic!("36f08ad7");
        };
        assert_eq!(
            u16::from(serde_json::from_str::<super::AdminPageLimit>("100").expect("d4d7c99a")),
            super::AdminPageLimit::MAX
        );
    }
    #[test]
    fn pagination_values_deserialize_from_url_query_strings() {
        let limit = <super::AdminPageLimit as serde::Deserialize>::deserialize(
            serde::de::value::StrDeserializer::<serde::de::value::Error>::new("100"),
        )
        .expect("a6aa5b42");
        let offset = <super::AdminPageOffset as serde::Deserialize>::deserialize(
            serde::de::value::StrDeserializer::<serde::de::value::Error>::new("42"),
        )
        .expect("799e47b0");
        assert_eq!(u16::from(limit), super::AdminPageLimit::MAX);
        assert_eq!(u32::from(offset), 42u32);
    }
    #[test]
    fn administrator_identifiers_require_positive_database_values() {
        let _user_error = super::AdminUserId::try_from(0i64).expect_err("6088ff6a");
        let _role_error = super::AdminRoleId::try_from(-1i64).expect_err("4406ffcc");
        let _permission_error = super::AdminPermissionId::try_from(0i64).expect_err("f5d79bb8");
        let _audit_error = super::AdminAuditLogId::try_from(-1i64).expect_err("3ca5fe6c");
    }
}
