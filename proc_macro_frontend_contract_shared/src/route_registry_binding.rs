#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryBinding {
    endpoint: crate::syn_route_registry_endpoint::SynRouteRegistryEndpoint,
    route: crate::syn_route_registry_route::SynRouteRegistryRoute,
}
