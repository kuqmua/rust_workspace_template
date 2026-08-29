#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract_macros::RouteCatalog,
)]
#[route_catalog(family = CommonRouteFamily, body_limit = constants_usize::ZERO)]
pub enum CommonRoute {
    #[route_catalog_route(crate::git_info_route::GitInfoRoute)]
    GitInfo,
    #[route_catalog_route(crate::health_route::HealthRoute)]
    Health,
    #[route_catalog_route(crate::health_check_route::HealthCheckRoute)]
    HealthCheck,
    #[route_catalog_route(crate::health_live_route::HealthLiveRoute)]
    HealthLive,
    #[route_catalog_route(crate::health_ready_route::HealthReadyRoute)]
    HealthReady,
}
impl CommonRoute {
    #[must_use]
    pub fn path(self) -> frontend_contract::contract_str::ContractStr {
        self.contract().path()
    }
}
