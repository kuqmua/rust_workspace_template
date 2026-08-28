pub fn validate_project_commit<'commit_lt, CommitIdTy>(
    commit_id: CommitIdTy,
) -> Result<(), crate::validate_project_commit_error::ValidateProjectCommitError>
where
    CommitIdTy: Into<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>,
{
    if crate::check_is_project_commit::check_is_project_commit(commit_id).0 {
        return Ok(());
    }
    Err(
        crate::validate_project_commit_error::ValidateProjectCommitError::from(
            crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value(),
        ),
    )
}
