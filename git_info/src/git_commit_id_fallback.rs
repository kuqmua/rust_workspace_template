#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub struct GitCommitIdFallback(Option<crate::git_commit_id::GitCommitId>);
