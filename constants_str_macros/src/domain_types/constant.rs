#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Constant {
    pub(super) name: super::SynIdent,
    pub(super) parts: super::ConstantParts,
    pub(super) visibility: Option<super::SynVisibility>,
}
