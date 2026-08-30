#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum StoragePathSegmentError {
    #[error("invalid storage path segment")]
    Invalid,
}
