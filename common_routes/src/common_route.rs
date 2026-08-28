use super::{GitInfoRoute, HealthCheckRoute, HealthLiveRoute, HealthReadyRoute, HealthRoute};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::RouteCatalog,
)]
#[route_catalog(family = CommonRouteFamily, body_limit = constants_usize::ZERO)]
pub enum CommonRoute {
    #[route_catalog_route(GitInfoRoute)]
    GitInfo,
    #[route_catalog_route(HealthRoute)]
    Health,
    #[route_catalog_route(HealthCheckRoute)]
    HealthCheck,
    #[route_catalog_route(HealthLiveRoute)]
    HealthLive,
    #[route_catalog_route(HealthReadyRoute)]
    HealthReady,
}
impl CommonRoute {
    #[must_use]
    pub fn path(self) -> frontend_contract::ContractStr {
        self.contract().path()
    }
}
