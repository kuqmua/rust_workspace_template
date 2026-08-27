use super::{AdminPagePathRef, admin_path_route_name};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::IntoStaticStr,
    frontend_contract::domain_types::UnitEnumCatalog,
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
    pub fn route_name(self) -> frontend_contract::domain_types::ContractStr {
        admin_path_route_name(AdminPagePathRef::from(self.get()))
    }
}
impl frontend_contract::domain_types::RouteRegistrationContract for AdminHtmlAction {
    fn method(self) -> frontend_contract::domain_types::RouteMethod {
        frontend_contract::domain_types::RouteMethod::Post
    }
    fn path(self) -> frontend_contract::domain_types::RegisteredRoutePath {
        frontend_contract::domain_types::RegisteredRoutePath::from(self.get())
    }
}
