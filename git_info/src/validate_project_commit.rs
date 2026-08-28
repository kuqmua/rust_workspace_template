use super::super::{
    GitCommitIdRef, ValidateProjectCommitError, check_is_project_commit,
    project_git_commit_link_ref_value,
};

pub fn validate_project_commit<'commit_lt, CommitIdTy>(
    commit_id: CommitIdTy,
) -> Result<(), ValidateProjectCommitError>
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    if check_is_project_commit(commit_id).0 {
        return Ok(());
    }
    Err(ValidateProjectCommitError::from(
        project_git_commit_link_ref_value(),
    ))
}
