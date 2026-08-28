use crate::domain_types::{GitCommitIdRef, GitCommitLinkOutputRefMut};

#[allow(
    clippy::single_call_fn,
    reason = "shared writer remains directly testable without duplicating commit-link assembly"
)]
pub(crate) fn write_git_commit_link<'commit_lt, CommitIdTy>(
    output: &mut GitCommitLinkOutputRefMut<'_>,
    commit_id: CommitIdTy,
) where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    output.0.push_str(constants_str::NAMING_GITHUB_URL);
    output.0.push_str(constants_str::GIT_INFO_TREE_SEGMENT);
    output.0.push_str(commit_id_ref.0);
}
