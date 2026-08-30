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
    commit: crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>,
}
impl<'commit_lt> ProjectGitInfo<'commit_lt> {
    #[must_use]
    pub const fn commit(&self) -> crate::git_commit_id_ref::GitCommitIdRef<'commit_lt> {
        self.commit
    }
}
impl<'commit_lt> From<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>
    for ProjectGitInfo<'commit_lt>
{
    fn from(value: crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>) -> Self {
        Self { commit: value }
    }
}
impl AsRef<str> for ProjectGitInfo<'_> {
    fn as_ref(&self) -> &str {
        <&str>::from(self.commit)
    }
}
