#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct CollectionsHashSet<Item>(pub(super) std::collections::HashSet<Item>);
