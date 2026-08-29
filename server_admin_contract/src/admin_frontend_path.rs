#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::IntoStaticStr,
)]
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
    #[strum(serialize = "/admin/roles/create")]
    RolesCreate,
    #[strum(serialize = "/admin/roles/manage")]
    RolesManage,
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
    #[strum(serialize = "/admin/users/create")]
    UsersCreate,
    #[strum(serialize = "/admin/users/manage")]
    UsersManage,
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
    fn method(self) -> frontend_contract::route_method::RouteMethod {
        frontend_contract::route_method::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::registered_route_path::RegisteredRoutePath {
        frontend_contract::registered_route_path::RegisteredRoutePath::from(self.get())
    }
}
