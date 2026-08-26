#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("bounded map length exceeds limit {0}")]
#[derive(newtype::FromInner)]
pub struct BoundedBTreeMapError(super::std_bounded_b_tree_map_len::StdBoundedBTreeMapLen);
