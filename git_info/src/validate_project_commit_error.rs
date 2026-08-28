#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ValidateProjectCommitError(
    pub(super) crate::project_git_commit_link_ref::ProjectGitCommitLinkRef,
);
