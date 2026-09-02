#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum CanBePrimaryKey {
    False,
    True,
}
