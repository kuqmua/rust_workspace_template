use super::{SynRouteRegistryEndpoint, SynRouteRegistryRoute};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryBinding {
    endpoint: SynRouteRegistryEndpoint,
    route: SynRouteRegistryRoute,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RouteRegistryBinding {
    #[allow(
        clippy::single_call_fn,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        endpoint: SynRouteRegistryEndpoint,
        route: SynRouteRegistryRoute,
    ) -> Self {
        Self { endpoint, route }
    }
}
