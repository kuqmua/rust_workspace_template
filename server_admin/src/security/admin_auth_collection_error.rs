#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("administrator authorization collection exceeds maximum length")]
pub(crate) struct AdminAuthCollectionError;
