#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct AdminPageSpec {
    #[getters(copy)]
    route: crate::admin_route::AdminRoute,
    #[getters(copy)]
    capability: crate::admin_page_capability::AdminPageCapability,
    metadata: crate::admin_page_metadata::AdminPageMetadata,
    #[getters(copy)]
    page: crate::admin_page::AdminPage,
    #[getters(skip)]
    path: crate::admin_frontend_path::AdminFrontendPath,
    #[getters(skip)]
    title: crate::admin_page_title::AdminPageTitle,
}
impl AdminPageSpec {
    pub(super) const fn new(
        admin_page_capability: crate::admin_page_capability::AdminPageCapability,
        admin_page_metadata: crate::admin_page_metadata::AdminPageMetadata,
        admin_page: crate::admin_page::AdminPage,
        admin_frontend_path: crate::admin_frontend_path::AdminFrontendPath,
        admin_route: crate::admin_route::AdminRoute,
        admin_page_title: crate::admin_page_title::AdminPageTitle,
    ) -> Self {
        Self {
            route: admin_route,
            capability: admin_page_capability,
            metadata: admin_page_metadata,
            page: admin_page,
            path: admin_frontend_path,
            title: admin_page_title,
        }
    }

    #[must_use]
    pub const fn client_mode(self) -> crate::admin_page_client_mode::AdminPageClientMode {
        *self.metadata.get_client_mode()
    }
    #[must_use]
    pub const fn navigation(self) -> Option<crate::admin_page_navigation::AdminPageNavigation> {
        self.metadata.get_navigation().copied()
    }
    #[must_use]
    pub const fn frontend_path(self) -> crate::admin_frontend_path::AdminFrontendPath {
        self.path
    }

    #[must_use]
    pub fn path(self) -> frontend_contract::contract_str::ContractStr {
        frontend_contract::contract_str::ContractStr::from(self.path.get())
    }
    #[must_use]
    pub fn route_name(self) -> frontend_contract::contract_str::ContractStr {
        crate::admin_path_route_name::admin_path_route_name(
            crate::admin_page_path_ref::AdminPagePathRef::from(self.path.get()),
        )
    }

    #[must_use]
    pub fn title(self) -> frontend_contract::contract_str::ContractStr {
        frontend_contract::contract_str::ContractStr::from(match self.title {
            crate::admin_page_title::AdminPageTitle::Api => constants_str::API_ALT,
            crate::admin_page_title::AdminPageTitle::Metrics => constants_str::METRICS_ALT,
            crate::admin_page_title::AdminPageTitle::Permissions => constants_str::PERMISSIONS,
            crate::admin_page_title::AdminPageTitle::Profile => constants_str::PROFILE,
            crate::admin_page_title::AdminPageTitle::Roles => constants_str::ROLES,
            crate::admin_page_title::AdminPageTitle::Sessions => constants_str::SESSIONS_ALT,
            crate::admin_page_title::AdminPageTitle::Settings => constants_str::SETTINGS,
            crate::admin_page_title::AdminPageTitle::Tables => constants_str::TABLES,
            crate::admin_page_title::AdminPageTitle::Users => constants_str::USERS,
            crate::admin_page_title::AdminPageTitle::Version => constants_str::VERSION_ALT,
        })
    }
}
