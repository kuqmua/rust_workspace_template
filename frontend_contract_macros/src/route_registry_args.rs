use crate::{
    SynExpr, SynRouteRegistryBindings, SynRouteRegistryFamily, SynRouteRegistrySchemas,
    SynRouteRegistryState,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryArgs {
    pub authenticated_security: SynExpr,
    pub bindings: SynRouteRegistryBindings,
    pub csrf_security: SynExpr,
    pub family: SynRouteRegistryFamily,
    pub schemas: SynRouteRegistrySchemas,
    pub state: SynRouteRegistryState,
}
