#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "dimension keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum Dimension {
    One,
    Two,
    Three,
    Four,
}
