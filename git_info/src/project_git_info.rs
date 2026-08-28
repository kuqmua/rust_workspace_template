#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::GitCommitIdRef;

#[derive(
    Debug,
    serde_derive::Serialize,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct ProjectGitInfo<'commit_lt> {
    pub(super) commit: GitCommitIdRef<'commit_lt>,
}
impl<'commit_lt> ProjectGitInfo<'commit_lt> {
    #[must_use]
    pub const fn commit(&self) -> GitCommitIdRef<'commit_lt> {
        self.commit
    }
}
impl<'commit_lt> From<GitCommitIdRef<'commit_lt>> for ProjectGitInfo<'commit_lt> {
    fn from(value: GitCommitIdRef<'commit_lt>) -> Self {
        Self { commit: value }
    }
}
impl AsRef<str> for ProjectGitInfo<'_> {
    fn as_ref(&self) -> &str {
        self.commit.0
    }
}
