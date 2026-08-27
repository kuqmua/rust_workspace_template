#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct BatchInvalidItems<InvalidItem>(pub(super) Vec<InvalidItem>);
