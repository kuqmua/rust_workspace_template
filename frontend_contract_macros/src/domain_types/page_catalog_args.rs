use super::SynIdent;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct PageCatalogArgs {
    pub(crate) inventory: SynIdent,
    pub(crate) path_ref: SynIdent,
    pub(crate) spec: SynIdent,
}
