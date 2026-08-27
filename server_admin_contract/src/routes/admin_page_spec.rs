use super::{
    AdminFrontendPath, AdminPage, AdminPageCapability, AdminPageClientMode, AdminPageMetadata,
    AdminPageNavigation, AdminPagePathRef, AdminPageTitle, AdminRoute, admin_path_route_name,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageSpec {
    route: AdminRoute,
    capability: AdminPageCapability,
    metadata: AdminPageMetadata,
    page: AdminPage,
    path: AdminFrontendPath,
    title: AdminPageTitle,
}
impl AdminPageSpec {
    pub(super) const fn new(
        capability: AdminPageCapability,
        metadata: AdminPageMetadata,
        page: AdminPage,
        path: AdminFrontendPath,
        route: AdminRoute,
        title: AdminPageTitle,
    ) -> Self {
        Self {
            route,
            capability,
            metadata,
            page,
            path,
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
    pub fn path(self) -> frontend_contract::domain_types::ContractStr {
        frontend_contract::domain_types::ContractStr::from(self.path.get())
    }
    #[must_use]
    pub fn route_name(self) -> frontend_contract::domain_types::ContractStr {
        admin_path_route_name(AdminPagePathRef::from(self.path.get()))
    }
    #[must_use]
    pub const fn route(self) -> AdminRoute {
        self.route
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::domain_types::ContractStr {
        frontend_contract::domain_types::ContractStr::from(match self.title {
            AdminPageTitle::Api => constants_str::API_ALT,
            AdminPageTitle::Metrics => constants_str::METRICS_ALT,
            AdminPageTitle::Permissions => constants_str::PERMISSIONS,
            AdminPageTitle::Profile => constants_str::PROFILE,
            AdminPageTitle::Roles => constants_str::ROLES,
            AdminPageTitle::Sessions => constants_str::SESSIONS_ALT,
            AdminPageTitle::Settings => constants_str::SETTINGS,
            AdminPageTitle::Tables => constants_str::TABLES,
            AdminPageTitle::Users => constants_str::USERS,
            AdminPageTitle::Version => constants_str::VERSION_ALT,
        })
    }
}
