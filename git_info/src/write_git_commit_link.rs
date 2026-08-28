use super::super::{GitCommitIdRef, GitCommitLinkOutputRefMut};

#[allow(clippy::single_call_fn)] // shared writer keeps link assembly consistent across builders and tests
pub(in crate::domain_types) fn write_git_commit_link<'commit_lt, CommitIdTy>(
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
