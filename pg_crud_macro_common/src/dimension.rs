#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum Dimension {
    One,
    Two,
    Three,
    Four,
}
