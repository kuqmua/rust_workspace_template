use super::Dimension;

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum DimensionIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dimension> for DimensionIndexNumber {
    fn from(v: &Dimension) -> Self {
        match &v {
            Dimension::One => Self::Zero,
            Dimension::Two => Self::One,
            Dimension::Three => Self::Two,
            Dimension::Four => Self::Three,
        }
    }
}
