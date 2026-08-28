use crate::domain_types::SynExpr;

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct PageCatalogPageArgs {
    capability: SynExpr,
    metadata: SynExpr,
    path: SynExpr,
    route: SynExpr,
    title: SynExpr,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl PageCatalogPageArgs {
    #[allow(
        clippy::single_call_fn,
        clippy::too_many_arguments,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        capability: SynExpr,
        metadata: SynExpr,
        path: SynExpr,
        route: SynExpr,
        title: SynExpr,
    ) -> Self {
        Self {
            capability,
            metadata,
            path,
            route,
            title,
        }
    }
}
