use crate::domain_types::{GitCommitIdRef, IsProjectCommit, project_git_info_value};

#[must_use]
pub fn check_is_project_commit<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> IsProjectCommit
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    IsProjectCommit::from(commit_id_ref.0 == project_git_info_value().commit.0)
}
