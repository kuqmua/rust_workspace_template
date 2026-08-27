#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct ListItems<Item>(pub(super) Vec<Item>);
