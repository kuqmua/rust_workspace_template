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
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::IntoStaticStr, utoipa::ToSchema)]
pub enum AdminPermission {
    #[strum(serialize = "audit_log:read")]
    AuditLogRead,
    #[strum(serialize = "metrics:read")]
    MetricsRead,
    #[strum(serialize = "openapi:read")]
    OpenApiRead,
    #[strum(serialize = "permissions:read")]
    PermissionsRead,
    #[strum(serialize = "role_permissions:create")]
    RolePermissionsCreate,
    #[strum(serialize = "role_permissions:delete")]
    RolePermissionsDelete,
    #[strum(serialize = "role_permissions:read")]
    RolePermissionsRead,
    #[strum(serialize = "role_permissions:update")]
    RolePermissionsUpdate,
    #[strum(serialize = "roles:create")]
    RolesCreate,
    #[strum(serialize = "roles:delete")]
    RolesDelete,
    #[strum(serialize = "roles:read")]
    RolesRead,
    #[strum(serialize = "roles:update")]
    RolesUpdate,
    #[strum(serialize = "system_settings:read")]
    SystemSettingsRead,
    #[strum(serialize = "system_settings:update")]
    SystemSettingsUpdate,
    #[strum(serialize = "user_roles:create")]
    UserRolesCreate,
    #[strum(serialize = "user_roles:delete")]
    UserRolesDelete,
    #[strum(serialize = "user_roles:read")]
    UserRolesRead,
    #[strum(serialize = "user_roles:update")]
    UserRolesUpdate,
    #[strum(serialize = "users:create")]
    UsersCreate,
    #[strum(serialize = "users:delete")]
    UsersDelete,
    #[strum(serialize = "users:read")]
    UsersRead,
    #[strum(serialize = "users:update")]
    UsersUpdate,
}
impl AdminPermission {
    pub const ALL: [Self; 22] = [
        Self::AuditLogRead,
        Self::MetricsRead,
        Self::OpenApiRead,
        Self::PermissionsRead,
        Self::RolePermissionsCreate,
        Self::RolePermissionsDelete,
        Self::RolePermissionsRead,
        Self::RolePermissionsUpdate,
        Self::RolesCreate,
        Self::RolesDelete,
        Self::RolesRead,
        Self::RolesUpdate,
        Self::SystemSettingsRead,
        Self::SystemSettingsUpdate,
        Self::UserRolesCreate,
        Self::UserRolesDelete,
        Self::UserRolesRead,
        Self::UserRolesUpdate,
        Self::UsersCreate,
        Self::UsersDelete,
        Self::UsersRead,
        Self::UsersUpdate,
    ];
    #[must_use]
    pub fn as_str(self) -> AdminPermissionStrRef<'static> {
        AdminPermissionStrRef::from(<&'static str>::from(self))
    }
}
impl serde::Serialize for AdminPermission {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serializer.serialize_str(self.as_str().as_ref())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPermissionTryFromStrError;
impl std::fmt::Display for AdminPermissionTryFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::UNKNOWN_ADMINISTRATOR_PERMISSION)
    }
}
impl std::error::Error for AdminPermissionTryFromStrError {}
impl TryFrom<&str> for AdminPermission {
    type Error = AdminPermissionTryFromStrError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::ALL
            .into_iter()
            .find(|permission| permission.as_str().as_ref() == value)
            .ok_or(AdminPermissionTryFromStrError)
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct AdminAuditDetailsBytes(usize);
impl From<usize> for AdminAuditDetailsBytes {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerdeJsonAdminAuditDetails(serde_json::Value);
impl AsRef<serde_json::Value> for SerdeJsonAdminAuditDetails {
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
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
impl From<SerdeJsonAdminAuditDetails> for serde_json::Value {
    fn from(value: SerdeJsonAdminAuditDetails) -> Self {
        value.0
    }
}
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(max = 8192, chars, serde, utoipa, validator = |value: &String| value
    .starts_with(AdminFrontendPath::Root.get()), description = "administrator default route")]
pub struct AdminDefaultRoute(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(max = 8192usize, min = 1usize, chars, serde, utoipa, validator = |value: &String| !value
    .trim()
    .is_empty(), description = "administrator site name")]
pub struct AdminSiteName(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator main logo"
)]
pub struct AdminMainLogo(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator organization contacts"
)]
pub struct AdminOrganizationContacts(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator organization name"
)]
pub struct AdminOrganizationName(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator primary color"
)]
pub struct AdminPrimaryColor(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator support URL"
)]
pub struct AdminSupportUrl(String);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefStr)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
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
        f.write_str("unknown admin table sort field")
    }
}
impl std::error::Error for AdminTableSortFieldTryFromKeyError {}
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
        key: &str,
    ) -> Result<Self, AdminTableSortFieldTryFromKeyError> {
        options
            .iter()
            .copied()
            .find(|option| option.key().as_ref() == key)
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
)]
pub struct AdminAuditLogId(i64);
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
pub struct AdminBool(bool);
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
    role_ids: Vec<AdminRoleId>,
}
impl AdminSetUserRolesReq {
    #[must_use]
    pub const fn from_ids(role_ids: Vec<AdminRoleId>) -> Self {
        Self { role_ids }
    }
    #[must_use]
    pub fn into_ids(self) -> Vec<AdminRoleId> {
        self.role_ids
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsReq {
    permission_ids: Vec<AdminPermissionId>,
}
impl AdminSetRolePermissionsReq {
    #[must_use]
    pub const fn from_ids(permission_ids: Vec<AdminPermissionId>) -> Self {
        Self { permission_ids }
    }
    #[must_use]
    pub fn into_ids(self) -> Vec<AdminPermissionId> {
        self.permission_ids
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminUserSummary {
    display_name: AdminDisplayName,
    id: AdminUserId,
    is_banned: AdminBool,
    login: AdminLogin,
}
impl AdminUserSummary {
    #[must_use]
    pub const fn new(
        display_name: AdminDisplayName,
        id: AdminUserId,
        is_banned: AdminBool,
        login: AdminLogin,
    ) -> Self {
        Self {
            display_name,
            id,
            is_banned,
            login,
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
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminRoleSummary {
    id: AdminRoleId,
    is_system: AdminBool,
    name: AdminRoleName,
}
impl AdminRoleSummary {
    #[must_use]
    pub const fn new(id: AdminRoleId, is_system: AdminBool, name: AdminRoleName) -> Self {
        Self {
            id,
            is_system,
            name,
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
    pub const fn resource(&self) -> &AdminText {
        &self.resource
    }
    #[must_use]
    pub const fn succeeded(&self) -> AdminBool {
        self.succeeded
    }
    #[must_use]
    pub const fn user_id(&self) -> Option<AdminUserId> {
        self.user_id
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
#[serde(deny_unknown_fields)]
pub struct AdminUpdateSettingsReq {
    default_admin_route: Option<AdminDefaultRoute>,
    main_logo: Option<AdminMainLogo>,
    organization_contacts: Option<AdminOrganizationContacts>,
    organization_name: Option<AdminOrganizationName>,
    primary_color: Option<AdminPrimaryColor>,
    site_name: Option<AdminSiteName>,
    support_url: Option<AdminSupportUrl>,
    tab_title: Option<AdminTabTitle>,
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

#[derive(Clone, Debug, newtype::BoundedString)]
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
    ) -> Self {
        Self {
            created_at,
            expires_at,
            id,
        }
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
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sign_out", path = "/auth/sign-out", request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSignOutRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sessions", path = "/auth/sessions", request = AdminNoBody, response = Vec<AdminSessionView>, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_session", path = "/auth/sessions/{session_id}", path_parameter = AdminSessionIdentifier, request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeSessionRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = AdminNoBody, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::UsersRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = AdminNoBody, response = Vec<AdminUserSummary>, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
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
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::RolesRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_roles", path = "/roles", request = AdminNoBody, response = Vec<AdminRoleSummary>, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
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
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::PermissionsRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_permissions", path = "/permissions", request = AdminNoBody, response = Vec<AdminPermissionSummary>, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListPermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::AuditLogRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "audit_log", path = "/audit-log", request = AdminNoBody, response = Vec<AdminAuditView>, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditLogRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::SystemSettingsRead), method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "settings", path = "/system-settings", request = AdminNoBody, response = AdminSettingsView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSettingsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(error_statuses = frontend_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES, authentication = admin_permission_requirement(AdminPermission::SystemSettingsUpdate), method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system-settings", request = AdminUpdateSettingsReq, response = AdminNoBody, success_status = frontend_contract::SuccessStatus::Code204, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateSettingsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::RouteFamily)]
#[route_family_body_limit(ADMIN_API_BODY_MAX_BYTES.get())]
#[route_family(
    AdminSignInRoute,
    AdminRefreshRoute,
    AdminMeRoute,
    AdminSignOutRoute,
    AdminSessionsRoute,
    AdminRevokeSessionRoute,
    AdminRevokeAllSessionsRoute,
    AdminListUsersRoute,
    AdminCreateUserRoute,
    AdminUpdateUserRoute,
    AdminDeleteUserRoute,
    AdminSetUserPasswordRoute,
    AdminSetUserBanRoute,
    AdminSetUserRolesRoute,
    AdminListRolesRoute,
    AdminCreateRoleRoute,
    AdminUpdateRoleRoute,
    AdminDeleteRoleRoute,
    AdminSetRolePermissionsRoute,
    AdminListPermissionsRoute,
    AdminAuditLogRoute,
    AdminSettingsRoute,
    AdminUpdateSettingsRoute
)]
pub struct AdminAuthenticationRouteFamily;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminRoute {
    Audit,
    CreateRole,
    CreateUser,
    DeleteRole(AdminRoleId),
    DeleteUser(AdminUserId),
    Me,
    Metrics,
    OpenApi,
    Permissions,
    Refresh,
    RevokeAllSessions,
    RevokeSession,
    Roles,
    SetRolePermissions(AdminRoleId),
    SetUserBan(AdminUserId),
    SetUserPassword(AdminUserId),
    SetUserRoles(AdminUserId),
    Settings,
    SignIn,
    SignOut,
    Sessions,
    UpdateRole(AdminRoleId),
    UpdateSettings,
    UpdateUser(AdminUserId),
    Users,
    Version,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
impl AsRef<str> for AdminRoutePath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl std::fmt::Display for AdminRoutePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminPagePathRef<'path_lt>(&'path_lt str);
impl<'path_lt> From<&'path_lt str> for AdminPagePathRef<'path_lt> {
    fn from(value: &'path_lt str) -> Self {
        Self(value)
    }
}
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
    #[strum(serialize = "/admin/roles")]
    Roles,
    #[strum(serialize = "/admin")]
    Root,
    #[strum(serialize = "/admin/sign-in")]
    SignIn,
    #[strum(serialize = "/admin/system-settings")]
    Settings,
    #[strum(serialize = "/admin/users")]
    Users,
    #[strum(serialize = "/admin/version")]
    Version,
}
impl AdminFrontendPath {
    pub const ALL_PAGES: [Self; 10] = [
        Self::Root,
        Self::SignIn,
        Self::Users,
        Self::Roles,
        Self::Permissions,
        Self::Audit,
        Self::Settings,
        Self::Metrics,
        Self::Version,
        Self::OpenApi,
    ];
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPage {
    Audit,
    Metrics,
    OpenApi,
    Permissions,
    Roles,
    Settings,
    Users,
    Version,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminPageCapability {
    Always,
    Swagger,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageSpec {
    capability: AdminPageCapability,
    page: AdminPage,
    path: AdminFrontendPath,
    route: AdminRoute,
    title: &'static str,
}
impl AdminPageSpec {
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
        frontend_contract::ContractStr::from(self.title)
    }
}
const ADMIN_PAGE_SPECS: [AdminPageSpec; 8] = [
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Users,
        path: AdminFrontendPath::Users,
        route: AdminRoute::Users,
        title: str_constants::USERS,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Roles,
        path: AdminFrontendPath::Roles,
        route: AdminRoute::Roles,
        title: str_constants::ROLES,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Permissions,
        path: AdminFrontendPath::Permissions,
        route: AdminRoute::Permissions,
        title: str_constants::PERMISSIONS,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Audit,
        path: AdminFrontendPath::Audit,
        route: AdminRoute::Audit,
        title: str_constants::AUDIT_LOG,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Settings,
        path: AdminFrontendPath::Settings,
        route: AdminRoute::Settings,
        title: str_constants::SETTINGS,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Metrics,
        path: AdminFrontendPath::Metrics,
        route: AdminRoute::Metrics,
        title: str_constants::METRICS_ALT,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Always,
        page: AdminPage::Version,
        path: AdminFrontendPath::Version,
        route: AdminRoute::Version,
        title: str_constants::VERSION_ALT,
    },
    AdminPageSpec {
        capability: AdminPageCapability::Swagger,
        page: AdminPage::OpenApi,
        path: AdminFrontendPath::OpenApi,
        route: AdminRoute::OpenApi,
        title: str_constants::API_ALT,
    },
];
impl AdminPage {
    pub fn all() -> impl Iterator<Item = Self> {
        ADMIN_PAGE_SPECS.iter().map(|spec| spec.page)
    }
    #[must_use]
    pub const fn specs() -> &'static [AdminPageSpec] {
        &ADMIN_PAGE_SPECS
    }
    #[must_use]
    pub fn from_path(path: AdminPagePathRef<'_>) -> Option<Self> {
        ADMIN_PAGE_SPECS
            .iter()
            .find(|spec| spec.path.get() == path.0)
            .map(|spec| spec.page)
    }
    #[must_use]
    pub const fn spec(self) -> AdminPageSpec {
        match self {
            Self::Users => ADMIN_PAGE_SPECS[0],
            Self::Roles => ADMIN_PAGE_SPECS[1],
            Self::Permissions => ADMIN_PAGE_SPECS[2],
            Self::Audit => ADMIN_PAGE_SPECS[3],
            Self::Settings => ADMIN_PAGE_SPECS[4],
            Self::Metrics => ADMIN_PAGE_SPECS[5],
            Self::Version => ADMIN_PAGE_SPECS[6],
            Self::OpenApi => ADMIN_PAGE_SPECS[7],
        }
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
    pub fn contract(self) -> frontend_contract::RouteContract {
        fn typed_contract<Route>() -> frontend_contract::RouteContract
        where
            Route: frontend_contract::TypedRoute,
        {
            Route::metadata().contract()
        }
        match self {
            Self::Audit => typed_contract::<AdminAuditLogRoute>(),
            Self::CreateRole => typed_contract::<AdminCreateRoleRoute>(),
            Self::CreateUser => typed_contract::<AdminCreateUserRoute>(),
            Self::DeleteRole(_) => typed_contract::<AdminDeleteRoleRoute>(),
            Self::DeleteUser(_) => typed_contract::<AdminDeleteUserRoute>(),
            Self::Me => typed_contract::<AdminMeRoute>(),
            Self::Permissions => typed_contract::<AdminListPermissionsRoute>(),
            Self::Refresh => typed_contract::<AdminRefreshRoute>(),
            Self::RevokeAllSessions => typed_contract::<AdminRevokeAllSessionsRoute>(),
            Self::RevokeSession => typed_contract::<AdminRevokeSessionRoute>(),
            Self::Roles => typed_contract::<AdminListRolesRoute>(),
            Self::SetRolePermissions(_) => typed_contract::<AdminSetRolePermissionsRoute>(),
            Self::SetUserBan(_) => typed_contract::<AdminSetUserBanRoute>(),
            Self::SetUserPassword(_) => typed_contract::<AdminSetUserPasswordRoute>(),
            Self::SetUserRoles(_) => typed_contract::<AdminSetUserRolesRoute>(),
            Self::Settings => typed_contract::<AdminSettingsRoute>(),
            Self::SignIn => typed_contract::<AdminSignInRoute>(),
            Self::SignOut => typed_contract::<AdminSignOutRoute>(),
            Self::Sessions => typed_contract::<AdminSessionsRoute>(),
            Self::UpdateRole(_) => typed_contract::<AdminUpdateRoleRoute>(),
            Self::UpdateSettings => typed_contract::<AdminUpdateSettingsRoute>(),
            Self::UpdateUser(_) => typed_contract::<AdminUpdateUserRoute>(),
            Self::Users => typed_contract::<AdminListUsersRoute>(),
            Self::Metrics => frontend_contract::RouteContract::new(
                admin_permission_requirement(AdminPermission::MetricsRead),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::METRICS),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::OpenApi => frontend_contract::RouteContract::new(
                admin_permission_requirement(AdminPermission::OpenApiRead),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::OPENAPI_JSON),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Version => frontend_contract::RouteContract::new(
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::COMMON_ROUTES_GIT_INFO),
                frontend_contract::SuccessStatus::Code200,
            ),
        }
    }
    #[must_use]
    pub fn path(self) -> AdminRoutePath {
        let suffix = match self {
            Self::DeleteRole(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminDeleteRoleRoute>(&id)
            }
            Self::UpdateRole(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminUpdateRoleRoute>(&id)
            }
            Self::SetRolePermissions(id) => frontend_contract::typed_parameterized_route_path::<
                AdminSetRolePermissionsRoute,
            >(&id),
            Self::DeleteUser(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminDeleteUserRoute>(&id)
            }
            Self::UpdateUser(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminUpdateUserRoute>(&id)
            }
            Self::SetUserBan(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminSetUserBanRoute>(&id)
            }
            Self::SetUserPassword(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminSetUserPasswordRoute>(&id)
            }
            Self::SetUserRoles(id) => {
                frontend_contract::typed_parameterized_route_path::<AdminSetUserRolesRoute>(&id)
            }
            Self::RevokeSession => {
                String::from(frontend_contract::typed_route_path::<AdminRevokeSessionRoute>())
            }
            Self::Version => String::from(str_constants::API_V1_GIT_INFO),
            value @ (Self::Audit
            | Self::CreateRole
            | Self::CreateUser
            | Self::Me
            | Self::Metrics
            | Self::OpenApi
            | Self::Permissions
            | Self::Refresh
            | Self::RevokeAllSessions
            | Self::Roles
            | Self::Settings
            | Self::SignIn
            | Self::SignOut
            | Self::Sessions
            | Self::UpdateSettings
            | Self::Users) => value.contract().path().as_ref().to_owned(),
        };
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
    fn authentication_route_family_has_valid_coverage() {
        let descriptors = <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
        assert_eq!(descriptors.len(), 23usize);
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
        let _empty_site_name_error =
            super::AdminSiteName::try_from(String::new()).expect_err("4cfb6820");
        let _blank_site_name_error =
            super::AdminSiteName::try_from(str_constants::SPACE.to_owned()).expect_err("b5fba19e");
        let _site_name =
            super::AdminSiteName::try_from(str_constants::ADMIN.to_owned()).expect("adb58327");
        let _default_route =
            super::AdminDefaultRoute::try_from(super::AdminFrontendPath::Users.get().to_owned())
                .expect("3582a0ec");
        let _invalid_route_error =
            super::AdminDefaultRoute::try_from(str_constants::ROUTE.to_owned())
                .expect_err("bb0d454a");
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
                str_constants::LOGIN,
            ),
            Ok(super::AdminTableSortField::UserLogin)
        );
        assert_eq!(
            super::AdminTableSortField::try_from_key(
                &super::AdminTableSortField::USER,
                str_constants::CREATED_AT,
            ),
            Err(super::AdminTableSortFieldTryFromKeyError)
        );
    }
}
