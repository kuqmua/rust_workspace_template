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
    description = "administrator setting text"
)]
pub struct AdminSettingText(String);
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
    default_admin_route: AdminSettingText,
    main_logo: Option<AdminSettingText>,
    organization_contacts: Option<AdminSettingText>,
    organization_name: Option<AdminSettingText>,
    primary_color: Option<AdminSettingText>,
    site_name: AdminSettingText,
    support_url: Option<AdminSettingText>,
    tab_title: Option<AdminSettingText>,
}
impl AdminSettingsView {
    #[must_use]
    pub const fn new(
        default_admin_route: AdminSettingText,
        main_logo: Option<AdminSettingText>,
        organization_contacts: Option<AdminSettingText>,
        organization_name: Option<AdminSettingText>,
        primary_color: Option<AdminSettingText>,
        site_name: AdminSettingText,
        support_url: Option<AdminSettingText>,
        tab_title: Option<AdminSettingText>,
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
    pub const fn default_admin_route(&self) -> &AdminSettingText {
        &self.default_admin_route
    }
    #[must_use]
    pub const fn main_logo(&self) -> Option<&AdminSettingText> {
        self.main_logo.as_ref()
    }
    #[must_use]
    pub const fn organization_contacts(&self) -> Option<&AdminSettingText> {
        self.organization_contacts.as_ref()
    }
    #[must_use]
    pub const fn organization_name(&self) -> Option<&AdminSettingText> {
        self.organization_name.as_ref()
    }
    #[must_use]
    pub const fn primary_color(&self) -> Option<&AdminSettingText> {
        self.primary_color.as_ref()
    }
    #[must_use]
    pub const fn site_name(&self) -> &AdminSettingText {
        &self.site_name
    }
    #[must_use]
    pub const fn support_url(&self) -> Option<&AdminSettingText> {
        self.support_url.as_ref()
    }
    #[must_use]
    pub const fn tab_title(&self) -> Option<&AdminSettingText> {
        self.tab_title.as_ref()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateSettingsReq {
    default_admin_route: Option<AdminSettingText>,
    main_logo: Option<AdminSettingText>,
    organization_contacts: Option<AdminSettingText>,
    organization_name: Option<AdminSettingText>,
    primary_color: Option<AdminSettingText>,
    site_name: Option<AdminSettingText>,
    support_url: Option<AdminSettingText>,
    tab_title: Option<AdminSettingText>,
}
impl AdminUpdateSettingsReq {
    #[must_use]
    pub const fn new(
        default_admin_route: Option<AdminSettingText>,
        main_logo: Option<AdminSettingText>,
        organization_contacts: Option<AdminSettingText>,
        organization_name: Option<AdminSettingText>,
        primary_color: Option<AdminSettingText>,
        site_name: Option<AdminSettingText>,
        support_url: Option<AdminSettingText>,
        tab_title: Option<AdminSettingText>,
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
        Option<AdminSettingText>,
        Option<AdminSettingText>,
        Option<AdminSettingText>,
        Option<AdminSettingText>,
        Option<AdminSettingText>,
        Option<AdminSettingText>,
        Option<AdminSettingText>,
        Option<AdminSettingText>,
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

#[derive(Clone, Debug, newtype::BoundedString)]
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
    access = frontend_contract::RouteAccess::Public,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "sign_in",
    path = "/auth/sign-in",
    request = AdminSignInReq,
    response = AdminSignInRes,
    transport = frontend_contract::PublicTransport,
)]
pub struct AdminSignInRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    access = frontend_contract::RouteAccess::Public,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "refresh",
    path = "/auth/refresh",
    request = AdminNoBody,
    response = AdminSignInRes,
    transport = frontend_contract::PublicTransport,
)]
pub struct AdminRefreshRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    access = frontend_contract::RouteAccess::Authenticated,
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "me",
    path = "/auth/me",
    request = AdminNoBody,
    response = AuthenticatedAdmin,
    transport = frontend_contract::AuthenticatedTransport,
)]
pub struct AdminMeRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sign_out", path = "/auth/sign-out", request = AdminNoBody, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSignOutRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "sessions", path = "/auth/sessions", request = AdminNoBody, response = Vec<AdminSessionView>, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_session", path = "/auth/sessions/{session_id}", request = AdminNoBody, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeSessionRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "revoke_all_sessions", path = "/auth/sessions", request = AdminNoBody, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminRevokeAllSessionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_users", path = "/users", request = AdminNoBody, response = Vec<AdminUserSummary>, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListUsersRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_user", path = "/users", request = AdminCreateUserReq, response = AdminCreateUserRes, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_user", path = "/users/{user_id}", request = AdminUpdateUserReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_user", path = "/users/{user_id}", request = AdminNoBody, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDeleteUserRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_password", path = "/users/{user_id}/password", request = AdminSetUserPasswordReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserPasswordRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_ban", path = "/users/{user_id}/ban", request = AdminSetUserBanReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserBanRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_user_roles", path = "/users/{user_id}/roles", request = AdminSetUserRolesReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetUserRolesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_roles", path = "/roles", request = AdminNoBody, response = Vec<AdminRoleSummary>, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListRolesRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Post, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "create_role", path = "/roles", request = AdminCreateRoleReq, response = AdminCreateRoleRes, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminCreateRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_role", path = "/roles/{role_id}", request = AdminUpdateRoleReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminUpdateRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Delete, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_role", path = "/roles/{role_id}", request = AdminNoBody, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDeleteRoleRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Put, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "set_role_permissions", path = "/roles/{role_id}/permissions", request = AdminSetRolePermissionsReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSetRolePermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_permissions", path = "/permissions", request = AdminNoBody, response = Vec<AdminPermissionSummary>, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminListPermissionsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "audit_log", path = "/audit-log", request = AdminNoBody, response = Vec<AdminAuditView>, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminAuditLogRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "settings", path = "/system-settings", request = AdminNoBody, response = AdminSettingsView, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminSettingsRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(access = frontend_contract::RouteAccess::Authenticated, method = frontend_contract::RouteMethod::Patch, mutation = frontend_contract::RouteMutation::Mutating, obligations = frontend_contract::AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "update_settings", path = "/system-settings", request = AdminUpdateSettingsReq, response = AdminNoBody, transport = frontend_contract::AuthenticatedTransport)]
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
impl AdminPage {
    pub const ALL: [Self; 8] = [
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
    pub fn from_path(path: AdminPagePathRef<'_>) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|page| page.path().as_ref() == path.0)
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(match self {
            Self::Audit => AdminFrontendPath::Audit.get(),
            Self::Metrics => AdminFrontendPath::Metrics.get(),
            Self::OpenApi => AdminFrontendPath::OpenApi.get(),
            Self::Permissions => AdminFrontendPath::Permissions.get(),
            Self::Roles => AdminFrontendPath::Roles.get(),
            Self::Settings => AdminFrontendPath::Settings.get(),
            Self::Users => AdminFrontendPath::Users.get(),
            Self::Version => AdminFrontendPath::Version.get(),
        })
    }
    #[must_use]
    pub const fn route(self) -> Option<AdminRoute> {
        match self {
            Self::Audit => Some(AdminRoute::Audit),
            Self::Metrics => Some(AdminRoute::Metrics),
            Self::OpenApi => Some(AdminRoute::OpenApi),
            Self::Permissions => Some(AdminRoute::Permissions),
            Self::Roles => Some(AdminRoute::Roles),
            Self::Settings => Some(AdminRoute::Settings),
            Self::Users => Some(AdminRoute::Users),
            Self::Version => Some(AdminRoute::Version),
        }
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::ContractStr {
        frontend_contract::ContractStr::from(match self {
            Self::Audit => str_constants::AUDIT_LOG,
            Self::Metrics => str_constants::METRICS_ALT,
            Self::OpenApi => str_constants::API_ALT,
            Self::Permissions => str_constants::PERMISSIONS,
            Self::Roles => str_constants::ROLES,
            Self::Settings => str_constants::SETTINGS,
            Self::Users => str_constants::USERS,
            Self::Version => str_constants::VERSION_ALT,
        })
    }
    #[must_use]
    pub fn authentication(self) -> frontend_contract::AuthenticationRequirement {
        self.route().map_or_else(
            || {
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::OpenApiRead.as_str().get(),
                    ),
                )
            },
            |route| route.contract().authentication(),
        )
    }
}
impl AdminRoute {
    #[must_use]
    pub fn contract(self) -> frontend_contract::RouteContract {
        let (authentication, method, mutation, path, status) = match self {
            Self::Audit => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::AuditLogRead.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminAuditLogRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::CreateRole => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::RolesCreate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminCreateRoleRoute>(),
                frontend_contract::SuccessStatus::Code201,
            ),
            Self::CreateUser => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::UsersCreate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminCreateUserRoute>(),
                frontend_contract::SuccessStatus::Code201,
            ),
            Self::DeleteRole(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::RolesDelete.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Delete,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminDeleteRoleRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::DeleteUser(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::UsersDelete.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Delete,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminDeleteUserRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Me => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminMeRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Metrics => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::MetricsRead.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::METRICS),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::OpenApi => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::OpenApiRead.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::OPENAPI_JSON),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Permissions => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::PermissionsRead.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminListPermissionsRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Refresh => (
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminRefreshRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::RevokeAllSessions => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Delete,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminRevokeAllSessionsRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::RevokeSession => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Delete,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminRevokeSessionRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Roles => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(AdminPermission::RolesRead.as_str().get()),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminListRolesRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::SetRolePermissions(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::RolePermissionsUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Put,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminSetRolePermissionsRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::SetUserBan(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::UsersUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminSetUserBanRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::SetUserPassword(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::UsersUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminSetUserPasswordRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::SetUserRoles(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::UserRolesUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Put,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminSetUserRolesRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Settings => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::SystemSettingsRead.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminSettingsRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::SignIn => (
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminSignInRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::SignOut => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminSignOutRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Sessions => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminSessionsRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::UpdateRole(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::RolesUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Patch,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminUpdateRoleRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::UpdateSettings => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::SystemSettingsUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Patch,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminUpdateSettingsRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::UpdateUser(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(
                        AdminPermission::UsersUpdate.as_str().get(),
                    ),
                ),
                frontend_contract::HttpMethod::Patch,
                frontend_contract::MutationKind::Mutating,
                frontend_contract::typed_route_path::<AdminUpdateUserRoute>(),
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Users => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from(AdminPermission::UsersRead.as_str().get()),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::typed_route_path::<AdminListUsersRoute>(),
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Version => (
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::COMMON_ROUTES_GIT_INFO),
                frontend_contract::SuccessStatus::Code200,
            ),
        };
        frontend_contract::RouteContract::new(authentication, method, mutation, path, status)
    }
    #[must_use]
    pub fn path(self) -> AdminRoutePath {
        let suffix = match self {
            Self::DeleteRole(id) | Self::UpdateRole(id) => {
                format!(
                    "{}/{id}",
                    frontend_contract::typed_route_path::<AdminListRolesRoute>()
                )
            }
            Self::SetRolePermissions(id) => {
                format!(
                    "{}/{id}/permissions",
                    frontend_contract::typed_route_path::<AdminListRolesRoute>()
                )
            }
            Self::DeleteUser(id) | Self::UpdateUser(id) => {
                format!(
                    "{}/{id}",
                    frontend_contract::typed_route_path::<AdminListUsersRoute>()
                )
            }
            Self::SetUserBan(id) => {
                format!(
                    "{}/{id}/ban",
                    frontend_contract::typed_route_path::<AdminListUsersRoute>()
                )
            }
            Self::SetUserPassword(id) => {
                format!(
                    "{}/{id}/password",
                    frontend_contract::typed_route_path::<AdminListUsersRoute>()
                )
            }
            Self::SetUserRoles(id) => {
                format!(
                    "{}/{id}/roles",
                    frontend_contract::typed_route_path::<AdminListUsersRoute>()
                )
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
}
