#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryBinding {
    endpoint: crate::syn_route_registry_endpoint::SynRouteRegistryEndpoint,
    route: crate::syn_route_registry_route::SynRouteRegistryRoute,
}
