use super::{
    SynExpr, SynRouteRegistryBindings, SynRouteRegistryFamily, SynRouteRegistrySchemas,
    SynRouteRegistryState,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryArgs {
    authenticated_security: SynExpr,
    bindings: SynRouteRegistryBindings,
    csrf_security: SynExpr,
    family: SynRouteRegistryFamily,
    schemas: SynRouteRegistrySchemas,
    state: SynRouteRegistryState,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RouteRegistryArgs {
    #[allow(
        clippy::single_call_fn,
        clippy::too_many_arguments,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        authenticated_security: SynExpr,
        bindings: SynRouteRegistryBindings,
        csrf_security: SynExpr,
        family: SynRouteRegistryFamily,
        schemas: SynRouteRegistrySchemas,
        state: SynRouteRegistryState,
    ) -> Self {
        Self {
            authenticated_security,
            bindings,
            csrf_security,
            family,
            schemas,
            state,
        }
    }
}
