#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::IntoStaticStr,
)]
pub enum AdminFrontendPath {
    #[strum(serialize = "/admin/access_sessions/{access_session_id}")]
    AccessSessionRead,
    #[strum(serialize = "/admin/audit_log/{audit_log_id}")]
    AuditLogRead,
    #[strum(serialize = "/admin/branding")]
    Branding,
    #[strum(serialize = "/admin/cleanup_status/{cleanup_status_id}")]
    CleanupStatusRead,
    #[strum(serialize = "/admin/health")]
    Health,
    #[strum(serialize = "/admin/login_attempts/{login_attempt_id}")]
    LoginAttemptRead,
    #[strum(serialize = "/admin/assets")]
    Assets,
    #[strum(serialize = "/admin/metrics")]
    Metrics,
    #[strum(serialize = "/admin/openapi.json")]
    OpenApiDocument,
    #[strum(serialize = "/admin/swagger_ui")]
    OpenApi,
    #[strum(serialize = "/admin/rules")]
    Rules,
    #[strum(serialize = "/admin/rules/{rule_id}")]
    RuleRead,
    #[strum(serialize = "/admin/profile")]
    Profile,
    #[strum(serialize = "/admin/rate_limits/{rate_limit_id}")]
    RateLimitRead,
    #[strum(serialize = "/admin/refresh_tokens/{refresh_token_id}")]
    RefreshTokenRead,
    #[strum(serialize = "/admin/roles")]
    Roles,
    #[strum(serialize = "/admin/roles/create")]
    RolesCreate,
    #[strum(serialize = "/admin/roles/manage")]
    RolesManage,
    #[strum(serialize = "/admin/roles/{role_id}")]
    RoleRead,
    #[strum(serialize = "/admin/role_rules/{role_rule_id}")]
    RoleRuleRead,
    #[strum(serialize = "/admin/sessions")]
    Sessions,
    #[strum(serialize = "/admin")]
    Root,
    #[strum(serialize = "/admin/sign_in")]
    SignIn,
    #[strum(serialize = "/admin/settings")]
    Settings,
    #[strum(serialize = "/admin/system_settings/{system_setting_id}")]
    SystemSettingRead,
    #[strum(serialize = "/admin/{table}")]
    Tables,
    #[strum(serialize = "/admin/users")]
    Users,
    #[strum(serialize = "/admin/users/create")]
    UsersCreate,
    #[strum(serialize = "/admin/users/manage")]
    UsersManage,
    #[strum(serialize = "/admin/users/{user_id}")]
    UserRead,
    #[strum(serialize = "/admin/user_roles/{user_role_id}")]
    UserRoleRead,
    #[strum(serialize = "/admin/version")]
    Version,
}
impl AdminFrontendPath {
    pub fn all_pages() -> impl Iterator<Item = Self> {
        [Self::Root, Self::SignIn].into_iter().chain(
            crate::admin_page::AdminPage::specs()
                .iter()
                .map(|spec| spec.frontend_path()),
        )
    }
    #[must_use]
    pub fn get(self) -> &'static str {
        <&'static str>::from(self)
    }
}
impl frontend_contract::route_registration_contract::RouteRegistrationContract
    for AdminFrontendPath
{
    fn registration_method(self) -> frontend_contract::route_method::RouteMethod {
        frontend_contract::route_method::RouteMethod::Get
    }
    fn registration_path(self) -> frontend_contract::contract_str::ContractStr {
        frontend_contract::contract_str::ContractStr::from(self.get())
    }
}
