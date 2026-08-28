use super::{StdBool, SynExpr, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteCatalogRouteArgs {
    pub(crate) contract: Option<SynExpr>,
    pub(crate) path: Option<SynExpr>,
    pub(crate) route: Option<SynType>,
    pub(crate) exclude_from_family: StdBool,
}
