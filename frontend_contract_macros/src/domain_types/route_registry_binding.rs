use super::{SynRouteRegistryEndpoint, SynRouteRegistryRoute};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteRegistryBinding {
    pub(crate) endpoint: SynRouteRegistryEndpoint,
    pub(crate) route: SynRouteRegistryRoute,
}
