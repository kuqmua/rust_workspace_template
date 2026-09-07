#[derive(
    Debug, Clone, Copy, PartialEq, Eq, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(crate) enum RunnerCliOutcome {
    Completed,
    Failed,
}
