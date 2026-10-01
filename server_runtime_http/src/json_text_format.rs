#[derive(Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum JsonTextFormat {
    Compact,
    Pretty,
}
