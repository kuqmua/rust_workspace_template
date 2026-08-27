use super::GitCommitId;

#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner,
)]
pub struct GitCommitIdFallback(pub(super) Option<GitCommitId>);
