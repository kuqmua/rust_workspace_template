use crate::{SynRouteRegistryEndpoint, SynRouteRegistryRoute};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryBinding {
    endpoint: SynRouteRegistryEndpoint,
    route: SynRouteRegistryRoute,
}
