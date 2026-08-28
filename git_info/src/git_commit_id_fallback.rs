#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner,
)]
pub struct GitCommitIdFallback(pub(super) Option<crate::git_commit_id::GitCommitId>);
