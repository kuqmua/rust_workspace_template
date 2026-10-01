#[derive(Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum ExternalTypeSegmentKind {
    Leaf,
    Root,
}
