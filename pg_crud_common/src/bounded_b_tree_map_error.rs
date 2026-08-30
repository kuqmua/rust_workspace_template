#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedBTreeMapError {
    #[error("bounded map length exceeds limit {0}")]
    TooLarge(super::std_bounded_b_tree_map_len::StdBoundedBTreeMapLen),
}
impl From<super::std_bounded_b_tree_map_len::StdBoundedBTreeMapLen> for BoundedBTreeMapError {
    fn from(value: super::std_bounded_b_tree_map_len::StdBoundedBTreeMapLen) -> Self {
        Self::TooLarge(value)
    }
}
