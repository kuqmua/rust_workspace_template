#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct CollectionsVecDeque<Item>(pub(super) std::collections::VecDeque<Item>);
