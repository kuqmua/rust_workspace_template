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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminApiBodyMaxBytes(usize);
impl AdminApiBodyMaxBytes {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
pub const ADMIN_API_BODY_MAX_BYTES: AdminApiBodyMaxBytes = AdminApiBodyMaxBytes(65_536usize);
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
    #[wire("mfa_recovery_codes:read")]
    MfaRecoveryCodesRead,
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
    #[wire("user_mfa:read")]
    UserMfaRead,
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
    #[wire("mfa_recovery_codes")]
    MfaRecoveryCodes,
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
    #[wire("user_mfa")]
    UserMfa,
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
impl AdminDataTable {
    #[must_use]
    pub const fn permission(self) -> AdminPermission {
        match self {
            Self::AccessSessions => AdminPermission::AccessSessionsRead,
            Self::AuditLog => AdminPermission::AuditLogRead,
            Self::CleanupStatus => AdminPermission::CleanupStatusRead,
            Self::LoginAttempts => AdminPermission::LoginAttemptsRead,
            Self::MfaRecoveryCodes => AdminPermission::MfaRecoveryCodesRead,
            Self::Permissions => AdminPermission::PermissionsRead,
            Self::RateLimits => AdminPermission::RateLimitsRead,
            Self::RefreshTokens => AdminPermission::RefreshTokensRead,
            Self::RolePermissions => AdminPermission::RolePermissionsRead,
            Self::Roles => AdminPermission::RolesRead,
            Self::SystemSettings => AdminPermission::SystemSettingsRead,
            Self::UserMfa => AdminPermission::UserMfaRead,
            Self::UserRoles => AdminPermission::UserRolesRead,
            Self::Users => AdminPermission::UsersRead,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminAuditDetailsTooLarge {
    actual_bytes: AdminAuditDetailsBytes,
}
impl AdminAuditDetailsTooLarge {
    #[must_use]
    pub const fn actual_bytes(self) -> AdminAuditDetailsBytes {
        self.actual_bytes
    }
    #[must_use]
    pub fn maximum_bytes(self) -> AdminAuditDetailsBytes {
        AdminAuditDetailsBytes::from(ADMIN_AUDIT_DETAILS_MAX_BYTES)
    }
}
impl std::fmt::Display for AdminAuditDetailsTooLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "administrator audit details contain {} bytes, maximum is {} bytes",
            self.actual_bytes.0, ADMIN_AUDIT_DETAILS_MAX_BYTES
        )
    }
}
impl std::error::Error for AdminAuditDetailsTooLarge {}
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
            return Err(AdminAuditDetailsTooLarge {
                actual_bytes: AdminAuditDetailsBytes::from(actual_bytes),
            });
        }
        Ok(Self(value))
    }
}
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(max = 8_192usize, chars, serde, utoipa, validator = |value: &String| AdminPage::from_path(AdminPagePathRef::from(value.as_str())).is_some(), description = "administrator default route")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminTableSortFieldTryFromKeyError;
impl std::fmt::Display for AdminTableSortFieldTryFromKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::UNKNOWN_ADMIN_TABLE_SORT_FIELD)
    }
}
impl std::error::Error for AdminTableSortFieldTryFromKeyError {}
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
    newtype::IntoInnerFrom,
)]
pub struct AdminUserId(i64);
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
    newtype::IntoInnerFrom,
)]
pub struct AdminRoleId(i64);
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
    newtype::IntoInnerFrom,
)]
pub struct AdminPermissionId(i64);
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
    newtype::IntoInnerFrom,
)]
pub struct AdminAuditLogId(i64);
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
pub struct AdminBool(bool);

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
#[serde(transparent)]
pub struct AdminPageOffset(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct AdminPageLimit(u16);
impl Default for AdminPageLimit {
    fn default() -> Self {
        Self(20u16)
    }
}
impl TryFrom<u16> for AdminPageLimit {
    type Error = AdminPageLimitError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (1u16..=100u16).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminPageLimitError)
        }
    }
}
impl<'de> serde::Deserialize<'de> for AdminPageLimit {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = u16::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl From<AdminPageLimit> for u16 {
    fn from(value: AdminPageLimit) -> Self {
        value.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminPageLimitError;
impl std::fmt::Display for AdminPageLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::ADMIN_PAGE_LIMIT_ERROR)
    }
}
impl std::error::Error for AdminPageLimitError {}

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
#[serde(transparent)]
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    display_name: AdminDisplayName,
    id: AdminUserId,
    login: AdminLogin,
    permissions: Vec<AdminPermissionValue>,
    roles: Vec<AdminRoleName>,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn new(
        display_name: AdminDisplayName,
        id: AdminUserId,
        login: AdminLogin,
        permissions: Vec<AdminPermissionValue>,
        roles: Vec<AdminRoleName>,
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
    pub const fn permissions(&self) -> &Vec<AdminPermissionValue> {
        &self.permissions
    }
    #[must_use]
    pub const fn login(&self) -> &AdminLogin {
        &self.login
    }
    #[must_use]
    pub const fn roles(&self) -> &[AdminRoleName] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn has_permission(&self, permission: AdminPermission) -> AdminBool {
        let required = permission.as_str();
        AdminBool::from(
            self.permissions
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
                .iter()
                .any(|value| value.as_ref() == required.as_ref()),
        })
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSignInRes {
    user: AuthenticatedAdmin,
}
impl AdminSignInRes {
    #[must_use]
    pub const fn new(user: AuthenticatedAdmin) -> Self {
        Self { user }
    }
    #[must_use]
    pub const fn user(&self) -> &AuthenticatedAdmin {
        &self.user
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateUserReq {
    display_name: AdminDisplayName,
    login: AdminLogin,
    password: AdminNewPassword,
}
impl AdminCreateUserReq {
    #[must_use]
    pub const fn new(
        display_name: AdminDisplayName,
        login: AdminLogin,
        password: AdminNewPassword,
    ) -> Self {
        Self {
            display_name,
            login,
            password,
        }
    }
    #[must_use]
    pub fn into_parts(self) -> (AdminDisplayName, AdminLogin, AdminNewPassword) {
        (self.display_name, self.login, self.password)
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminCreateUserRes {
    id: AdminUserId,
}
impl AdminCreateUserRes {
    #[must_use]
    pub const fn new(id: AdminUserId) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateUserReq {
    display_name: Option<AdminDisplayName>,
    login: Option<AdminLogin>,
}
impl AdminUpdateUserReq {
    #[must_use]
    pub const fn new(display_name: Option<AdminDisplayName>, login: Option<AdminLogin>) -> Self {
        Self {
            display_name,
            login,
        }
    }
    #[must_use]
    pub fn into_parts(self) -> (Option<AdminDisplayName>, Option<AdminLogin>) {
        (self.display_name, self.login)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserPasswordReq {
    password: AdminNewPassword,
}
impl AdminSetUserPasswordReq {
    #[must_use]
    pub const fn new(password: AdminNewPassword) -> Self {
        Self { password }
    }
    #[must_use]
    pub fn into_password(self) -> AdminNewPassword {
        self.password
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminChangeOwnPasswordReq {
    current_password: AdminPassword,
    new_password: AdminNewPassword,
    revoke_other_sessions: AdminBool,
}
impl AdminChangeOwnPasswordReq {
    #[must_use]
    pub const fn new(
        current_password: AdminPassword,
        new_password: AdminNewPassword,
        revoke_other_sessions: AdminBool,
    ) -> Self {
        Self {
            current_password,
            new_password,
            revoke_other_sessions,
        }
    }
    #[must_use]
    pub fn into_parts(self) -> (AdminPassword, AdminNewPassword, AdminBool) {
        (
            self.current_password,
            self.new_password,
            self.revoke_other_sessions,
        )
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserBanReq {
    is_banned: AdminBool,
}
impl AdminSetUserBanReq {
    #[must_use]
    pub const fn new(is_banned: AdminBool) -> Self {
        Self { is_banned }
    }
    #[must_use]
    pub const fn is_banned(self) -> AdminBool {
        self.is_banned
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateRoleReq {
    name: AdminRoleName,
}
impl AdminCreateRoleReq {
    #[must_use]
    pub const fn new(name: AdminRoleName) -> Self {
        Self { name }
    }
    #[must_use]
    pub fn into_name(self) -> AdminRoleName {
        self.name
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminCreateRoleRes {
    id: AdminRoleId,
}
impl AdminCreateRoleRes {
    #[must_use]
    pub const fn new(id: AdminRoleId) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateRoleReq {
    name: AdminRoleName,
}
impl AdminUpdateRoleReq {
    #[must_use]
    pub const fn new(name: AdminRoleName) -> Self {
        Self { name }
    }
    #[must_use]
    pub fn into_name(self) -> AdminRoleName {
        self.name
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserRolesReq {
    expected_role_ids: Vec<AdminRoleId>,
    role_ids: Vec<AdminRoleId>,
}
impl AdminSetUserRolesReq {
    #[must_use]
    pub const fn new(expected_role_ids: Vec<AdminRoleId>, role_ids: Vec<AdminRoleId>) -> Self {
        Self {
            expected_role_ids,
            role_ids,
        }
    }
    #[must_use]
    pub fn into_parts(self) -> (Vec<AdminRoleId>, Vec<AdminRoleId>) {
        (self.expected_role_ids, self.role_ids)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsReq {
    expected_permission_ids: Vec<AdminPermissionId>,
    permission_ids: Vec<AdminPermissionId>,
}
impl AdminSetRolePermissionsReq {
    #[must_use]
    pub const fn new(
        expected_permission_ids: Vec<AdminPermissionId>,
        permission_ids: Vec<AdminPermissionId>,
    ) -> Self {
        Self {
            expected_permission_ids,
            permission_ids,
        }
    }
    #[must_use]
    pub fn into_parts(self) -> (Vec<AdminPermissionId>, Vec<AdminPermissionId>) {
        (self.expected_permission_ids, self.permission_ids)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminUserSummary {
    display_name: AdminDisplayName,
    id: AdminUserId,
    is_banned: AdminBool,
    login: AdminLogin,
    #[serde(default)]
    role_ids: Vec<AdminRoleId>,
}
impl AdminUserSummary {
    #[must_use]
    pub const fn new(
        display_name: AdminDisplayName,
        id: AdminUserId,
        is_banned: AdminBool,
        login: AdminLogin,
        role_ids: Vec<AdminRoleId>,
    ) -> Self {
        Self {
            display_name,
            id,
            is_banned,
            login,
            role_ids,
        }
    }
    #[must_use]
    pub const fn display_name(&self) -> &AdminDisplayName {
        &self.display_name
    }
    #[must_use]
    pub const fn id(&self) -> AdminUserId {
        self.id
    }
    #[must_use]
    pub const fn is_banned(&self) -> AdminBool {
        self.is_banned
    }
    #[must_use]
    pub const fn login(&self) -> &AdminLogin {
        &self.login
    }
    #[must_use]
    pub const fn role_ids(&self) -> &[AdminRoleId] {
        self.role_ids.as_slice()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminRoleSummary {
    id: AdminRoleId,
    is_system: AdminBool,
    name: AdminRoleName,
    #[serde(default)]
    permission_ids: Vec<AdminPermissionId>,
}
impl AdminRoleSummary {
    #[must_use]
    pub const fn new(
        id: AdminRoleId,
        is_system: AdminBool,
        name: AdminRoleName,
        permission_ids: Vec<AdminPermissionId>,
    ) -> Self {
        Self {
            id,
            is_system,
            name,
            permission_ids,
        }
    }
    #[must_use]
    pub const fn id(&self) -> AdminRoleId {
        self.id
    }
    #[must_use]
    pub const fn is_system(&self) -> AdminBool {
        self.is_system
    }
    #[must_use]
    pub const fn name(&self) -> &AdminRoleName {
        &self.name
    }
    #[must_use]
    pub const fn permission_ids(&self) -> &[AdminPermissionId] {
        self.permission_ids.as_slice()
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
    pub const fn name(&self) -> &AdminPermissionValue {
        &self.name
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminUsersPage {
    items: Vec<AdminUserSummary>,
    roles: Vec<AdminRoleSummary>,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
impl AdminUsersPage {
    #[must_use]
    pub const fn new(
        items: Vec<AdminUserSummary>,
        roles: Vec<AdminRoleSummary>,
        total: AdminPageTotal,
    ) -> Self {
        Self {
            items,
            roles,
            total,
        }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminUserSummary] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn total(&self) -> AdminPageTotal {
        self.total
    }
    #[must_use]
    pub const fn roles(&self) -> &[AdminRoleSummary] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn into_items(self) -> Vec<AdminUserSummary> {
        self.items
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminRolesPage {
    items: Vec<AdminRoleSummary>,
    permissions: Vec<AdminPermissionSummary>,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
impl AdminRolesPage {
    #[must_use]
    pub const fn new(
        items: Vec<AdminRoleSummary>,
        permissions: Vec<AdminPermissionSummary>,
        total: AdminPageTotal,
    ) -> Self {
        Self {
            items,
            permissions,
            total,
        }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminRoleSummary] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn total(&self) -> AdminPageTotal {
        self.total
    }
    #[must_use]
    pub const fn permissions(&self) -> &[AdminPermissionSummary] {
        self.permissions.as_slice()
    }
    #[must_use]
    pub fn into_items(self) -> Vec<AdminRoleSummary> {
        self.items
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminPermissionsPage {
    items: Vec<AdminPermissionSummary>,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
impl AdminPermissionsPage {
    #[must_use]
    pub const fn new(items: Vec<AdminPermissionSummary>, total: AdminPageTotal) -> Self {
        Self { items, total }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminPermissionSummary] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn total(&self) -> AdminPageTotal {
        self.total
    }
    #[must_use]
    pub fn into_items(self) -> Vec<AdminPermissionSummary> {
        self.items
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminAuditView {
    action: AdminText,
    created_at: AdminAuditTimestamp,
    details: Option<SerdeJsonAdminAuditDetails>,
    id: AdminAuditLogId,
    resource: AdminText,
    resource_id: Option<AdminText>,
    succeeded: AdminBool,
    user_id: Option<AdminUserId>,
    user_login: Option<AdminLogin>,
}
impl AdminAuditView {
    #[must_use]
    pub const fn new(
        action: AdminText,
        created_at: AdminAuditTimestamp,
        details: Option<SerdeJsonAdminAuditDetails>,
        id: AdminAuditLogId,
        resource: AdminText,
        resource_id: Option<AdminText>,
        succeeded: AdminBool,
        user_id: Option<AdminUserId>,
        user_login: Option<AdminLogin>,
    ) -> Self {
        Self {
            action,
            created_at,
            details,
            id,
            resource,
            resource_id,
            succeeded,
            user_id,
            user_login,
        }
    }
    #[must_use]
    pub const fn action(&self) -> &AdminText {
        &self.action
    }
    #[must_use]
    pub const fn created_at(&self) -> &AdminAuditTimestamp {
        &self.created_at
    }
    #[must_use]
    pub const fn details(&self) -> Option<&SerdeJsonAdminAuditDetails> {
        self.details.as_ref()
    }
    #[must_use]
    pub const fn id(&self) -> AdminAuditLogId {
        self.id
    }
    #[must_use]
    pub const fn resource(&self) -> &AdminText {
        &self.resource
    }
    #[must_use]
    pub const fn resource_id(&self) -> Option<&AdminText> {
        self.resource_id.as_ref()
    }
    #[must_use]
    pub const fn succeeded(&self) -> AdminBool {
        self.succeeded
    }
    #[must_use]
    pub const fn user_id(&self) -> Option<AdminUserId> {
        self.user_id
    }
    #[must_use]
    pub const fn user_login(&self) -> Option<&AdminLogin> {
        self.user_login.as_ref()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminAuditCursor {
    created_at: AdminAuditTimestamp,
    id: AdminAuditLogId,
}
impl AdminAuditCursor {
    #[must_use]
    pub const fn new(created_at: AdminAuditTimestamp, id: AdminAuditLogId) -> Self {
        Self { created_at, id }
    }
    #[must_use]
    pub const fn created_at(&self) -> &AdminAuditTimestamp {
        &self.created_at
    }
    #[must_use]
    pub const fn id(&self) -> AdminAuditLogId {
        self.id
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminAuditPage {
    items: Vec<AdminAuditView>,
    #[schema(inline)]
    next_cursor: Option<AdminAuditCursor>,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
#[derive(Clone, Debug)]
pub struct AdminAuditHtmlQuery {
    action: Option<AdminText>,
    resource: Option<AdminText>,
    resource_id: Option<AdminText>,
    user_login: Option<AdminLogin>,
}
impl AdminAuditHtmlQuery {
    #[must_use]
    pub const fn new(
        action: Option<AdminText>,
        resource: Option<AdminText>,
        resource_id: Option<AdminText>,
        user_login: Option<AdminLogin>,
    ) -> Self {
        Self {
            action,
            resource,
            resource_id,
            user_login,
        }
    }
    #[must_use]
    pub const fn action(&self) -> Option<&AdminText> {
        self.action.as_ref()
    }
    #[must_use]
    pub const fn resource(&self) -> Option<&AdminText> {
        self.resource.as_ref()
    }
    #[must_use]
    pub const fn resource_id(&self) -> Option<&AdminText> {
        self.resource_id.as_ref()
    }
    #[must_use]
    pub const fn user_login(&self) -> Option<&AdminLogin> {
        self.user_login.as_ref()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminDataRow {
    values: Vec<AdminText>,
}
impl AdminDataRow {
    #[must_use]
    pub const fn new(values: Vec<AdminText>) -> Self {
        Self { values }
    }
    #[must_use]
    pub const fn values(&self) -> &[AdminText] {
        self.values.as_slice()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminDataTableView {
    columns: Vec<AdminText>,
    items: Vec<AdminDataRow>,
    table: AdminDataTable,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
impl AdminDataTableView {
    #[must_use]
    pub const fn new(
        columns: Vec<AdminText>,
        items: Vec<AdminDataRow>,
        table: AdminDataTable,
        total: AdminPageTotal,
    ) -> Self {
        Self {
            columns,
            items,
            table,
            total,
        }
    }
    #[must_use]
    pub const fn columns(&self) -> &[AdminText] {
        self.columns.as_slice()
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminDataRow] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn table(&self) -> AdminDataTable {
        self.table
    }
    #[must_use]
    pub const fn total(&self) -> AdminPageTotal {
        self.total
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminDataTableCatalog {
    items: Vec<AdminDataTable>,
}
impl AdminDataTableCatalog {
    #[must_use]
    pub const fn new(items: Vec<AdminDataTable>) -> Self {
        Self { items }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminDataTable] {
        self.items.as_slice()
    }
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminAuditExport {
    #[schema(value_type = String, max_length = 262144)]
    csv: AdminAuditExportCsv,
}
impl AdminAuditExport {
    #[must_use]
    pub const fn new(csv: AdminAuditExportCsv) -> Self {
        Self { csv }
    }
    #[must_use]
    pub const fn csv(&self) -> &AdminAuditExportCsv {
        &self.csv
    }
}
impl AdminAuditPage {
    #[must_use]
    pub const fn new(
        items: Vec<AdminAuditView>,
        next_cursor: Option<AdminAuditCursor>,
        total: AdminPageTotal,
    ) -> Self {
        Self {
            items,
            next_cursor,
            total,
        }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminAuditView] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn next_cursor(&self) -> Option<&AdminAuditCursor> {
        self.next_cursor.as_ref()
    }
    #[must_use]
    pub const fn total(&self) -> AdminPageTotal {
        self.total
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSettingsView {
    default_admin_route: AdminDefaultRoute,
    main_logo: Option<AdminMainLogo>,
    organization_contacts: Option<AdminOrganizationContacts>,
    organization_name: Option<AdminOrganizationName>,
    primary_color: Option<AdminPrimaryColor>,
    site_name: AdminSiteName,
    support_url: Option<AdminSupportUrl>,
    tab_title: Option<AdminTabTitle>,
}
impl AdminSettingsView {
    #[must_use]
    pub const fn new(
        default_admin_route: AdminDefaultRoute,
        main_logo: Option<AdminMainLogo>,
        organization_contacts: Option<AdminOrganizationContacts>,
        organization_name: Option<AdminOrganizationName>,
        primary_color: Option<AdminPrimaryColor>,
        site_name: AdminSiteName,
        support_url: Option<AdminSupportUrl>,
        tab_title: Option<AdminTabTitle>,
    ) -> Self {
        Self {
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
    pub const fn default_admin_route(&self) -> &AdminDefaultRoute {
        &self.default_admin_route
    }
    #[must_use]
    pub const fn main_logo(&self) -> Option<&AdminMainLogo> {
        self.main_logo.as_ref()
    }
    #[must_use]
    pub const fn organization_contacts(&self) -> Option<&AdminOrganizationContacts> {
        self.organization_contacts.as_ref()
    }
    #[must_use]
    pub const fn organization_name(&self) -> Option<&AdminOrganizationName> {
        self.organization_name.as_ref()
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
    clear: Vec<AdminOptionalSetting>,
    default_admin_route: Option<AdminDefaultRoute>,
    main_logo: Option<AdminMainLogo>,
    organization_contacts: Option<AdminOrganizationContacts>,
    organization_name: Option<AdminOrganizationName>,
    primary_color: Option<AdminPrimaryColor>,
    site_name: Option<AdminSiteName>,
    support_url: Option<AdminSupportUrl>,
    tab_title: Option<AdminTabTitle>,
}
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminOptionalSetting {
    MainLogo,
    OrganizationContacts,
    OrganizationName,
    PrimaryColor,
    SupportUrl,
    TabTitle,
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
        clear: Vec<AdminOptionalSetting>,
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
        Vec<AdminOptionalSetting>,
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
                || !self.clear.is_empty(),
        )
    }
    #[must_use]
    pub fn is_valid(&self) -> AdminBool {
        let unique = self
            .clear
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        AdminBool::from(
            unique.len() == self.clear.len()
                && self.clear.len() <= 6usize
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
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminApiErrorCode {
    AuthenticationFailed,
    AuthorizationFailed,
    Conflict,
    CsrfFailed,
    InternalError,
    RateLimited,
    ValidationFailed,
}
impl std::fmt::Display for AdminApiErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AuthenticationFailed => str_constants::AUTHENTICATION_FAILED,
            Self::AuthorizationFailed => str_constants::AUTHORIZATION_FAILED,
            Self::Conflict => str_constants::CONFLICT,
            Self::CsrfFailed => str_constants::CSRF_VALIDATION_FAILED,
            Self::InternalError => str_constants::INTERNAL_ERROR,
            Self::RateLimited => str_constants::RATE_LIMITED,
            Self::ValidationFailed => str_constants::VALIDATION_FAILED,
        })
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminApiErrorBody {
    code: AdminApiErrorCode,
}
impl AdminApiErrorBody {
    #[must_use]
    pub const fn new(code: AdminApiErrorCode) -> Self {
        Self { code }
    }
    #[must_use]
    pub const fn code(self) -> AdminApiErrorCode {
        self.code
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
    items: Vec<AdminSessionView>,
    #[schema(value_type = u64)]
    total: AdminPageTotal,
}
impl AdminSessionsPage {
    #[must_use]
    pub const fn new(items: Vec<AdminSessionView>, total: AdminPageTotal) -> Self {
        Self { items, total }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminSessionView] {
        self.items.as_slice()
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
    error_statuses = frontend_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
    openapi_operation_id = "sign_in",
    path = "/auth/sign-in",
    request = AdminSignInReq,
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
    error_statuses = frontend_contract::PUBLIC_REFRESH_ROUTE_ERROR_STATUSES,
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
    error_statuses = frontend_contract::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = AdminNoBody,
    response = AuthenticatedAdmin,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::AuthenticatedTransport,
)]
pub struct AdminMeRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "change_own_password", path = "/auth/password", request = AdminChangeOwnPasswordReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminChangeOwnPasswordRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sign_out", path = "/auth/sign-out", request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSignOutRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sessions", path = "/auth/sessions", request = AdminNoBody, response = AdminSessionsPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_session", path = "/auth/sessions/{session_id}", path_parameter = AdminSessionIdentifier, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeSessionRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = AdminNoBody, response = AdminUsersPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListUsersRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersCreate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_user", path = "/users", request = AdminCreateUserReq, response = AdminCreateUserRes, success_status = frontend_contract::SuccessStatus::Code201, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_user", path = "/users/{user_id}", path_parameter = AdminUserId, request = AdminUpdateUserReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersDelete), method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_user", path = "/users/{user_id}", path_parameter = AdminUserId, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDeleteUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_password", path = "/users/{user_id}/password", path_parameter = AdminUserId, request = AdminSetUserPasswordReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserPasswordRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersUpdate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_ban", path = "/users/{user_id}/ban", path_parameter = AdminUserId, request = AdminSetUserBanReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserBanRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UserRolesUpdate), method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_roles", path = "/users/{user_id}/roles", path_parameter = AdminUserId, request = AdminSetUserRolesReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserRolesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::RolesRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_roles", path = "/roles", request = AdminNoBody, response = AdminRolesPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListRolesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::RolesCreate), method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_role", path = "/roles", request = AdminCreateRoleReq, response = AdminCreateRoleRes, success_status = frontend_contract::SuccessStatus::Code201, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::RolesUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_role", path = "/roles/{role_id}", path_parameter = AdminRoleId, request = AdminUpdateRoleReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::RolesDelete), method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_role", path = "/roles/{role_id}", path_parameter = AdminRoleId, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDeleteRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::RolePermissionsUpdate), method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_role_permissions", path = "/roles/{role_id}/permissions", path_parameter = AdminRoleId, request = AdminSetRolePermissionsReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetRolePermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::PermissionsRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_permissions", path = "/permissions", request = AdminNoBody, response = AdminPermissionsPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListPermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::AuditLogRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "audit_log", path = "/audit-log", request = AdminNoBody, response = AdminAuditPage, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditLogRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::AuditLogExport), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "export_audit_log", path = "/audit-log/export", request = AdminNoBody, response = AdminAuditExport, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditExportRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::PUBLIC_READ_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Public, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "branding", path = "/branding", request = AdminNoBody, response = AdminBrandingView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::PublicTransport)]
pub struct AdminBrandingRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::TablesRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_data_tables", path = "/tables", request = AdminNoBody, response = AdminDataTableCatalog, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDataTablesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "read_data_table", path = "/tables/{table}", path_parameter = AdminDataTable, request = AdminNoBody, response = AdminDataTableView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDataTableRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::SystemSettingsRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "settings", path = "/system-settings", request = AdminNoBody, response = AdminSettingsView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSettingsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::SystemSettingsUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system-settings", request = AdminUpdateSettingsReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateSettingsRoute;

#[derive(Clone, Copy, Debug, PartialEq, Eq, frontend_contract::RouteCatalog)]
#[route_catalog(
    family = AdminAuthenticationRouteFamily,
    body_limit = ADMIN_API_BODY_MAX_BYTES.get(),
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
    #[strum(serialize = "/admin/audit-log")]
    Audit,
    #[strum(serialize = "/admin/metrics")]
    Metrics,
    #[strum(serialize = "/admin/openapi.json")]
    OpenApiDocument,
    #[strum(serialize = "/admin/swagger-ui")]
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
    #[strum(serialize = "/admin/sign-in")]
    SignIn,
    #[strum(serialize = "/admin/system-settings")]
    Settings,
    #[strum(serialize = "/admin/tables")]
    Tables,
    #[strum(serialize = "/admin/users")]
    Users,
    #[strum(serialize = "/admin/version")]
    Version,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::IntoStaticStr)]
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
    #[strum(serialize = "/admin/actions/sign-in")]
    SignIn,
    #[strum(serialize = "/admin/actions/sign-out")]
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
    pub const ALL: [Self; 15] = [
        Self::ProfilePassword,
        Self::RoleCreate,
        Self::RoleDelete,
        Self::RolePermissions,
        Self::RoleUpdate,
        Self::SessionRevoke,
        Self::SettingsUpdate,
        Self::SignIn,
        Self::SignOut,
        Self::UserBan,
        Self::UserCreate,
        Self::UserDelete,
        Self::UserPassword,
        Self::UserRoles,
        Self::UserUpdate,
    ];
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
}
impl AdminFrontendPath {
    pub const ALL_PAGES: [Self; 13] = [
        Self::Root,
        Self::SignIn,
        Self::Users,
        Self::Roles,
        Self::Sessions,
        Self::Permissions,
        Self::Profile,
        Self::Audit,
        Self::Settings,
        Self::Tables,
        Self::Metrics,
        Self::Version,
        Self::OpenApi,
    ];
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
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
        path = AdminFrontendPath::Users,
        route = AdminRoute::Users,
        title = AdminPageTitle::Users,
    )]
    Users,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Roles,
        route = AdminRoute::Roles,
        title = AdminPageTitle::Roles,
    )]
    Roles,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Permissions,
        route = AdminRoute::Permissions,
        title = AdminPageTitle::Permissions,
    )]
    Permissions,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Audit,
        route = AdminRoute::Audit,
        title = AdminPageTitle::AuditLog,
    )]
    Audit,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Settings,
        route = AdminRoute::Settings,
        title = AdminPageTitle::Settings,
    )]
    Settings,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Tables,
        route = AdminRoute::DataTables,
        title = AdminPageTitle::Tables,
    )]
    Tables,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Sessions,
        route = AdminRoute::Sessions,
        title = AdminPageTitle::Sessions,
    )]
    Sessions,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Metrics,
        route = AdminRoute::Metrics,
        title = AdminPageTitle::Metrics,
    )]
    Metrics,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Version,
        route = AdminRoute::Version,
        title = AdminPageTitle::Version,
    )]
    Version,
    #[page_catalog_page(
        capability = AdminPageCapability::Always,
        path = AdminFrontendPath::Profile,
        route = AdminRoute::ChangeOwnPassword,
        title = AdminPageTitle::Profile,
    )]
    Profile,
    #[page_catalog_page(
        capability = AdminPageCapability::Swagger,
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
enum AdminPageTitle {
    Api,
    AuditLog,
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
    page: AdminPage,
    path: AdminFrontendPath,
    route: AdminRoute,
    title: AdminPageTitle,
}
impl AdminPageSpec {
    const fn new(
        capability: AdminPageCapability,
        page: AdminPage,
        path: AdminFrontendPath,
        route: AdminRoute,
        title: AdminPageTitle,
    ) -> Self {
        Self {
            capability,
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
    pub const fn page(self) -> AdminPage {
        self.page
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(self.path.get())
    }
    #[must_use]
    pub const fn route(self) -> AdminRoute {
        self.route
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(match self.title {
            AdminPageTitle::Api => str_constants::API_ALT,
            AdminPageTitle::AuditLog => str_constants::AUDIT_LOG,
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
impl AdminPage {
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
        let suffix = String::from(self.catalog_path());
        if matches!(self, Self::Version) {
            AdminRoutePath::try_from(suffix).unwrap_or_default()
        } else {
            AdminRoutePath::try_from(format!(
                "{}{}{suffix}",
                str_constants::API_V1,
                AdminFrontendPath::Root.get()
            ))
            .unwrap_or_default()
        }
    }
}
#[cfg(test)]
mod tests {
    fn assert_rejects_unknown_field<Value>(json: &str)
    where
        Value: serde::de::DeserializeOwned,
    {
        assert!(serde_json::from_str::<Value>(json).is_err());
    }
    #[test]
    fn authenticated_admin_checks_permissions_and_page_access() {
        let admin = super::AuthenticatedAdmin::new(
            super::AdminDisplayName::try_from(str_constants::ADMIN.to_owned()).expect("67f10787"),
            super::AdminUserId::from(1i64),
            super::AdminLogin::try_from(str_constants::ROOT.to_owned()).expect("ced445ee"),
            vec![
                super::AdminPermissionValue::try_from(
                    super::AdminPermission::UsersRead.as_str().get().to_owned(),
                )
                .expect("837c99bb"),
            ],
            Vec::new(),
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
        assert_eq!(descriptors.len(), 28usize);
        assert_eq!(
            frontend_contract::validate_route_coverage(&descriptors),
            Ok(())
        );
        assert_eq!(
            <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
                .map(frontend_contract::RouteBodyLimit::get),
            Some(super::ADMIN_API_BODY_MAX_BYTES.get())
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
            Vec::new(),
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
            Vec::new(),
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
            vec![super::AdminOptionalSetting::MainLogo],
        );
        assert!(bool::from(clear_logo.has_fields()));
        assert!(bool::from(clear_logo.is_valid()));
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
        let route = super::AdminRoute::SetUserBan(super::AdminUserId::from(7));
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
            Some(super::AdminAuditDetailsTooLarge {
                actual_bytes: super::AdminAuditDetailsBytes::from(
                    super::ADMIN_AUDIT_DETAILS_MAX_BYTES.saturating_add(2usize),
                ),
            })
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
        assert_eq!(super::AdminDataTable::ALL.len(), 14usize);
        super::AdminDataTable::ALL.into_iter().for_each(|table| {
            assert_eq!(
                super::AdminDataTable::try_from(table.to_string()).expect("0596134b"),
                table
            );
            assert!(table.permission().as_str().get().ends_with(":read"));
        });
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
            100u16
        );
    }
}
