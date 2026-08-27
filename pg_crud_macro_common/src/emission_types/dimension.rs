//todo maybe reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum Dimension {
    One,
    Two,
    Three,
    Four,
}
