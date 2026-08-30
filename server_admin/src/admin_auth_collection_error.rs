#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub(crate) enum AdminAuthCollectionError {
    #[error("administrator authorization collection exceeds maximum length")]
    TooLarge,
}
