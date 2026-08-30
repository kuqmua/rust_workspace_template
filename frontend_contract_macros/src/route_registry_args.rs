#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryArgs {
    pub authenticated_security: crate::contract_syn_expr::ContractSynExpr,
    pub bindings: crate::syn_route_registry_bindings::SynRouteRegistryBindings,
    pub csrf_security: crate::contract_syn_expr::ContractSynExpr,
    pub family: crate::syn_route_registry_family::SynRouteRegistryFamily,
    pub schemas: crate::syn_route_registry_schemas::SynRouteRegistrySchemas,
    pub state: crate::syn_route_registry_state::SynRouteRegistryState,
}
