use crate::domain_types::{StdBool, SynExpr, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogRouteArgs {
    contract: Option<SynExpr>,
    path: Option<SynExpr>,
    route: Option<SynType>,
    exclude_from_family: StdBool,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RouteCatalogRouteArgs {
    #[allow(
        clippy::too_many_arguments,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        contract: Option<SynExpr>,
        path: Option<SynExpr>,
        route: Option<SynType>,
        exclude_from_family: StdBool,
    ) -> Self {
        Self {
            contract,
            path,
            route,
            exclude_from_family,
        }
    }
}
