#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RunMode {
    Apply,
    DryRun,
}
