#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_frontend_contract_derive_page_catalog::PageCatalog,
)]
#[page_catalog(
    spec = crate::admin_page_spec::AdminPageSpec,
    path_ref = crate::admin_page_path_ref::AdminPagePathRef,
    inventory = ADMIN_PAGE_SPECS,
)]
pub enum AdminPage {
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(crate::admin_page_client_mode::AdminPageClientMode::CsrTableQuery, None),
        path = crate::admin_frontend_path::AdminFrontendPath::Users,
        route = crate::admin_route::AdminRoute::Users,
        title = crate::admin_page_title::AdminPageTitle::Users,
    )]
    Users,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(crate::admin_page_client_mode::AdminPageClientMode::CsrTableQuery, None),
        path = crate::admin_frontend_path::AdminFrontendPath::Roles,
        route = crate::admin_route::AdminRoute::Roles,
        title = crate::admin_page_title::AdminPageTitle::Roles,
    )]
    Roles,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(crate::admin_page_client_mode::AdminPageClientMode::CsrTableQuery, None),
        path = crate::admin_frontend_path::AdminFrontendPath::Permissions,
        route = crate::admin_route::AdminRoute::Permissions,
        title = crate::admin_page_title::AdminPageTitle::Permissions,
    )]
    Permissions,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(
            crate::admin_page_client_mode::AdminPageClientMode::Csr,
            Some(crate::admin_page_navigation::AdminPageNavigation::Settings),
        ),
        path = crate::admin_frontend_path::AdminFrontendPath::Settings,
        route = crate::admin_route::AdminRoute::Settings,
        title = crate::admin_page_title::AdminPageTitle::Settings,
    )]
    Settings,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(crate::admin_page_client_mode::AdminPageClientMode::Csr, None),
        path = crate::admin_frontend_path::AdminFrontendPath::Tables,
        route = crate::admin_route::AdminRoute::DataTables,
        title = crate::admin_page_title::AdminPageTitle::Tables,
    )]
    Tables,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(
            crate::admin_page_client_mode::AdminPageClientMode::Csr,
            Some(crate::admin_page_navigation::AdminPageNavigation::Sessions),
        ),
        path = crate::admin_frontend_path::AdminFrontendPath::Sessions,
        route = crate::admin_route::AdminRoute::Sessions,
        title = crate::admin_page_title::AdminPageTitle::Sessions,
    )]
    Sessions,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(
            crate::admin_page_client_mode::AdminPageClientMode::Ssr,
            Some(crate::admin_page_navigation::AdminPageNavigation::Metrics),
        ),
        path = crate::admin_frontend_path::AdminFrontendPath::Metrics,
        route = crate::admin_route::AdminRoute::Metrics,
        title = crate::admin_page_title::AdminPageTitle::Metrics,
    )]
    Metrics,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(
            crate::admin_page_client_mode::AdminPageClientMode::Ssr,
            Some(crate::admin_page_navigation::AdminPageNavigation::Version),
        ),
        path = crate::admin_frontend_path::AdminFrontendPath::Version,
        route = crate::admin_route::AdminRoute::Version,
        title = crate::admin_page_title::AdminPageTitle::Version,
    )]
    Version,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Always,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(
            crate::admin_page_client_mode::AdminPageClientMode::Csr,
            Some(crate::admin_page_navigation::AdminPageNavigation::Profile),
        ),
        path = crate::admin_frontend_path::AdminFrontendPath::Profile,
        route = crate::admin_route::AdminRoute::ChangeOwnPassword,
        title = crate::admin_page_title::AdminPageTitle::Profile,
    )]
    Profile,
    #[page_catalog_page(
        capability = crate::admin_page_capability::AdminPageCapability::Swagger,
        metadata = crate::admin_page_metadata::AdminPageMetadata::new(
            crate::admin_page_client_mode::AdminPageClientMode::Ssr,
            Some(crate::admin_page_navigation::AdminPageNavigation::OpenApi),
        ),
        path = crate::admin_frontend_path::AdminFrontendPath::OpenApi,
        route = crate::admin_route::AdminRoute::OpenApi,
        title = crate::admin_page_title::AdminPageTitle::Api,
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
    pub fn supports_csr(self) -> crate::admin_bool::AdminBool {
        self.spec().client_mode().supports_csr()
    }
    #[must_use]
    pub fn uses_table_query(self) -> crate::admin_bool::AdminBool {
        self.spec().client_mode().uses_table_query()
    }
    #[must_use]
    pub fn path(self) -> frontend_contract::contract_str::ContractStr {
        self.spec().path()
    }
    #[must_use]
    pub const fn route(self) -> Option<crate::admin_route::AdminRoute> {
        Some(self.spec().route())
    }
    #[must_use]
    pub fn title(self) -> frontend_contract::contract_str::ContractStr {
        self.spec().title()
    }
    #[must_use]
    pub fn authentication(
        self,
    ) -> frontend_contract::authentication_requirement::AuthenticationRequirement {
        self.spec().route().contract().authentication()
    }
}
