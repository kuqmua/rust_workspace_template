use super::SynExpr;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct PageCatalogPageArgs {
    pub(crate) capability: SynExpr,
    pub(crate) metadata: SynExpr,
    pub(crate) path: SynExpr,
    pub(crate) route: SynExpr,
    pub(crate) title: SynExpr,
}
