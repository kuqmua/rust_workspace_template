// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum DimensionIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&crate::dimension::Dimension> for DimensionIndexNumber {
    fn from(dimension: &crate::dimension::Dimension) -> Self {
        match &dimension {
            crate::dimension::Dimension::One => Self::Zero,
            crate::dimension::Dimension::Two => Self::One,
            crate::dimension::Dimension::Three => Self::Two,
            crate::dimension::Dimension::Four => Self::Three,
        }
    }
}
