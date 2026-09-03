#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryArgs {
    authenticated_security: crate::contract_syn_expr::ContractSynExpr,
    bindings: crate::syn_route_registry_bindings::SynRouteRegistryBindings,
    csrf_security: crate::contract_syn_expr::ContractSynExpr,
    family: crate::syn_route_registry_family::SynRouteRegistryFamily,
    schemas: crate::syn_route_registry_schemas::SynRouteRegistrySchemas,
    state: crate::syn_route_registry_state::SynRouteRegistryState,
}
