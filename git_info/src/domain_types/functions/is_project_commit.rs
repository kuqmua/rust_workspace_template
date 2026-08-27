use super::super::{GitCommitIdRef, IsProjectCommit, project_git_info};

#[must_use]
pub fn is_project_commit<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> IsProjectCommit
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    IsProjectCommit::from(commit_id_ref.0 == project_git_info().commit.0)
}
