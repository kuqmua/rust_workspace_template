use super::{SynExpr, SynIdent};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RouteCatalogArgs {
    pub(crate) body_limit: SynExpr,
    pub(crate) family: SynIdent,
}
