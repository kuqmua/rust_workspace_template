#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "dimension index number keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum DimensionIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&crate::dimension::Dimension> for DimensionIndexNumber {
    fn from(value: &crate::dimension::Dimension) -> Self {
        match &value {
            crate::dimension::Dimension::One => Self::Zero,
            crate::dimension::Dimension::Two => Self::One,
            crate::dimension::Dimension::Three => Self::Two,
            crate::dimension::Dimension::Four => Self::Three,
        }
    }
}
