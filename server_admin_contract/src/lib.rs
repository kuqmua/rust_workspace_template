#![allow(clippy::arbitrary_source_item_ordering)] // DTO implementations keep constructors adjacent to their accessors and route metadata grouped by concern
const ADMIN_API_PREFIX: &str = "/api/v1/admin";
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator API text"
)]
#[newtype(as_ref_owned, display, into_inner)]
pub struct AdminText(String);
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(max = 128, chars, serde, utoipa, description = "administrator login")]
#[newtype(as_ref_owned, display, into_inner)]
pub struct AdminLogin(String);
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 256,
    chars,
    serde,
    utoipa,
    description = "administrator display name"
)]
#[newtype(as_ref_owned, display, into_inner)]
pub struct AdminDisplayName(String);
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 128,
    chars,
    serde,
    utoipa,
    description = "administrator role name"
)]
#[newtype(as_ref_owned, display, into_inner)]
pub struct AdminRoleName(String);
#[derive(Clone, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 1024usize,
    min = 1usize,
    chars,
    serde,
    utoipa,
    description = "administrator password"
)]
#[newtype(as_ref_owned, into_inner)]
pub struct AdminPassword(String);
impl std::fmt::Debug for AdminPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminPassword").field(&"[REDACTED]").finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 128,
    chars,
    serde,
    utoipa,
    description = "administrator permission"
)]
#[newtype(as_ref_owned, display, into_inner)]
pub struct AdminPermissionValue(String);
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator audit timestamp"
)]
#[newtype(as_ref_owned, display, into_inner)]
pub struct AdminAuditTimestamp(String);
#[derive(
    Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct SerdeJsonAdminAuditDetails(serde_json::Value);
#[derive(Clone, Debug, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 8192,
    chars,
    serde,
    utoipa,
    description = "administrator setting text"
)]
#[newtype(as_ref_owned, display, into_inner)]
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
    newtype::Newtype,
)]
#[newtype(display, from_inner, into_inner_from)]
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
    newtype::Newtype,
)]
#[newtype(display, from_inner, into_inner_from)]
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
    newtype::Newtype,
)]
#[newtype(display, from_inner, into_inner_from)]
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
    newtype::Newtype,
)]
#[newtype(display, from_inner)]
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
    newtype::Newtype,
)]
#[newtype(display, from_inner, into_inner_from)]
pub struct AdminBool(bool);
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
pub struct AdminCreateUserReq {
    display_name: AdminDisplayName,
    login: AdminLogin,
    password: AdminPassword,
}
impl AdminCreateUserReq {
    #[must_use]
    pub const fn new(
        display_name: AdminDisplayName,
        login: AdminLogin,
        password: AdminPassword,
    ) -> Self {
        Self {
            display_name,
            login,
            password,
        }
    }
    #[must_use]
    pub fn into_parts(self) -> (AdminDisplayName, AdminLogin, AdminPassword) {
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
pub struct AdminSetUserPasswordReq {
    password: AdminPassword,
}
impl AdminSetUserPasswordReq {
    #[must_use]
    pub const fn new(password: AdminPassword) -> Self {
        Self { password }
    }
    #[must_use]
    pub fn into_password(self) -> AdminPassword {
        self.password
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
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
pub enum AdminApiErCode {
    AuthenticationFailed,
    AuthorizationFailed,
    Conflict,
    CsrfFailed,
    InternalError,
    RateLimited,
    ValidationFailed,
}
impl std::fmt::Display for AdminApiErCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AuthenticationFailed => "authentication failed",
            Self::AuthorizationFailed => "authorization failed",
            Self::Conflict => "conflict",
            Self::CsrfFailed => "CSRF validation failed",
            Self::InternalError => "internal error",
            Self::RateLimited => "rate limited",
            Self::ValidationFailed => "validation failed",
        })
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminApiErBody {
    code: AdminApiErCode,
}
impl AdminApiErBody {
    #[must_use]
    pub const fn new(code: AdminApiErCode) -> Self {
        Self { code }
    }
    #[must_use]
    pub const fn code(self) -> AdminApiErCode {
        self.code
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminRoute {
    Audit,
    CreateRole,
    CreateUser,
    DeleteRole(AdminRoleId),
    DeleteUser(AdminUserId),
    Me,
    Metrics,
    Permissions,
    Refresh,
    Roles,
    SetRolePermissions(AdminRoleId),
    SetUserBan(AdminUserId),
    SetUserPassword(AdminUserId),
    SetUserRoles(AdminUserId),
    Settings,
    SignIn,
    SignOut,
    UpdateRole(AdminRoleId),
    UpdateSettings,
    UpdateUser(AdminUserId),
    Users,
    Version,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AdminRoutePath(Box<str>);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminRoutePathEr {
    TooLong,
}
impl std::fmt::Display for AdminRoutePathEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong => f.write_str("administrator route path is too long"),
        }
    }
}
impl TryFrom<String> for AdminRoutePath {
    type Error = AdminRoutePathEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 8192usize {
            Err(AdminRoutePathEr::TooLong)
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
            Self::Audit => "/admin/audit-log",
            Self::Metrics => "/admin/metrics",
            Self::OpenApi => "/admin/swagger-ui",
            Self::Permissions => "/admin/permissions",
            Self::Roles => "/admin/roles",
            Self::Settings => "/admin/system-settings",
            Self::Users => "/admin/users",
            Self::Version => "/admin/version",
        })
    }
    #[must_use]
    pub const fn route(self) -> Option<AdminRoute> {
        match self {
            Self::Audit => Some(AdminRoute::Audit),
            Self::Metrics => Some(AdminRoute::Metrics),
            Self::OpenApi => None,
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
            Self::Audit => "Audit log",
            Self::Metrics => "Metrics",
            Self::OpenApi => "API",
            Self::Permissions => "Permissions",
            Self::Roles => "Roles",
            Self::Settings => "Settings",
            Self::Users => "Users",
            Self::Version => "Version",
        })
    }
    #[must_use]
    pub fn authentication(self) -> frontend_contract::AuthenticationRequirement {
        self.route().map_or_else(
            || {
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("openapi:read"),
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
                    frontend_contract::ContractStr::from("audit_log:read"),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/audit-log",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::CreateRole => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("roles:create"),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                "/roles",
                frontend_contract::SuccessStatus::Code201,
            ),
            Self::CreateUser => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("users:create"),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                "/users",
                frontend_contract::SuccessStatus::Code201,
            ),
            Self::DeleteRole(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("roles:delete"),
                ),
                frontend_contract::HttpMethod::Delete,
                frontend_contract::MutationKind::Mutating,
                "/roles/{id}",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::DeleteUser(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("users:delete"),
                ),
                frontend_contract::HttpMethod::Delete,
                frontend_contract::MutationKind::Mutating,
                "/users/{id}",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Me => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/auth/me",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Metrics => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("metrics:read"),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/metrics",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Permissions => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("permissions:read"),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/permissions",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Refresh => (
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                "/auth/refresh",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Roles => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("roles:read"),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/roles",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::SetRolePermissions(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("role_permissions:update"),
                ),
                frontend_contract::HttpMethod::Put,
                frontend_contract::MutationKind::Mutating,
                "/roles/{id}/permissions",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::SetUserBan(_) | Self::SetUserPassword(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("users:update"),
                ),
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                "/users/{id}/custom",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::SetUserRoles(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("user_roles:update"),
                ),
                frontend_contract::HttpMethod::Put,
                frontend_contract::MutationKind::Mutating,
                "/users/{id}/roles",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Settings => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("system_settings:read"),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/system-settings",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::SignIn => (
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                "/auth/sign-in",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::SignOut => (
                frontend_contract::AuthenticationRequirement::Authenticated,
                frontend_contract::HttpMethod::Post,
                frontend_contract::MutationKind::Mutating,
                "/auth/sign-out",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::UpdateRole(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("roles:update"),
                ),
                frontend_contract::HttpMethod::Patch,
                frontend_contract::MutationKind::Mutating,
                "/roles/{id}",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::UpdateSettings => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("system_settings:update"),
                ),
                frontend_contract::HttpMethod::Patch,
                frontend_contract::MutationKind::Mutating,
                "/system-settings",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::UpdateUser(_) => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("users:update"),
                ),
                frontend_contract::HttpMethod::Patch,
                frontend_contract::MutationKind::Mutating,
                "/users/{id}",
                frontend_contract::SuccessStatus::Code204,
            ),
            Self::Users => (
                frontend_contract::AuthenticationRequirement::Permission(
                    frontend_contract::ContractStr::from("users:read"),
                ),
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/users",
                frontend_contract::SuccessStatus::Code200,
            ),
            Self::Version => (
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                "/git_info",
                frontend_contract::SuccessStatus::Code200,
            ),
        };
        frontend_contract::RouteContract::new(
            authentication,
            method,
            mutation,
            frontend_contract::ContractStr::from(path),
            status,
        )
    }
    #[must_use]
    pub fn path(self) -> AdminRoutePath {
        let suffix = match self {
            Self::DeleteRole(id) | Self::UpdateRole(id) => format!("/roles/{id}"),
            Self::SetRolePermissions(id) => format!("/roles/{id}/permissions"),
            Self::DeleteUser(id) | Self::UpdateUser(id) => format!("/users/{id}"),
            Self::SetUserBan(id) => format!("/users/{id}/ban"),
            Self::SetUserPassword(id) => format!("/users/{id}/password"),
            Self::SetUserRoles(id) => format!("/users/{id}/roles"),
            Self::Version => String::from("/api/v1/git_info"),
            value @ (Self::Audit
            | Self::CreateRole
            | Self::CreateUser
            | Self::Me
            | Self::Metrics
            | Self::Permissions
            | Self::Refresh
            | Self::Roles
            | Self::Settings
            | Self::SignIn
            | Self::SignOut
            | Self::UpdateSettings
            | Self::Users) => value.contract().path().as_ref().to_owned(),
        };
        if matches!(self, Self::Version) {
            AdminRoutePath::try_from(suffix).unwrap_or_default()
        } else {
            AdminRoutePath::try_from(format!("{ADMIN_API_PREFIX}{suffix}")).unwrap_or_default()
        }
    }
}
#[cfg(test)]
mod tests {
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
                frontend_contract::ContractStr::from("users:update")
            )
        );
    }
    #[test]
    fn password_debug_is_redacted() {
        let password = super::AdminPassword::try_from(String::from("secret")).expect("9f3f5164");
        assert!(!format!("{password:?}").contains("secret"));
    }
}
