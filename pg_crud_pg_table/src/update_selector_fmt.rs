#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum UpdateSelectorFmt {
    Eq,
    InList,
}
