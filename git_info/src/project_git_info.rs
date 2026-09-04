#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    Debug,
    serde_derive::Serialize,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct ProjectGitInfo<'commit_lt> {
    #[getters(copy)]
    commit: crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>,
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
