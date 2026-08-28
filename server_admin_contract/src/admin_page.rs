use super::{
    AdminFrontendPath, AdminPageCapability, AdminPageClientMode, AdminPageMetadata,
    AdminPageNavigation, AdminPagePathRef, AdminPageSpec, AdminPageTitle, AdminRoute,
};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    frontend_contract::PageCatalog,
)]
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
    pub fn supports_csr(self) -> crate::domain_types::AdminBool {
        self.spec().client_mode().supports_csr()
    }
    #[must_use]
    pub fn uses_table_query(self) -> crate::domain_types::AdminBool {
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
