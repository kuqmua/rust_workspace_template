#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::GitCommitId;

#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner,
)]
pub struct GitCommitIdFallback(pub(super) Option<GitCommitId>);
