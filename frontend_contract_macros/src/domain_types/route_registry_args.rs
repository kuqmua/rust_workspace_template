use super::{
    SynExpr, SynRouteRegistryBindings, SynRouteRegistryFamily, SynRouteRegistrySchemas,
    SynRouteRegistryState,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteRegistryArgs {
    pub(crate) authenticated_security: SynExpr,
    pub(crate) bindings: SynRouteRegistryBindings,
    pub(crate) csrf_security: SynExpr,
    pub(crate) family: SynRouteRegistryFamily,
    pub(crate) schemas: SynRouteRegistrySchemas,
    pub(crate) state: SynRouteRegistryState,
}
