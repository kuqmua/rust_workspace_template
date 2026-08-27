#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Fragment {
    pub(super) name: super::SynIdent,
    pub(super) value: super::SynLitStr,
}
