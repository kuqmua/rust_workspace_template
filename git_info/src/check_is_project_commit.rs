#[must_use]
pub fn check_is_project_commit<'commit_lt, CommitIdTy>(
    commit_id_ty: CommitIdTy,
) -> crate::is_project_commit::IsProjectCommit
where
    CommitIdTy: Into<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id_ty.into();
    crate::is_project_commit::IsProjectCommit::from(
        commit_id_ref
            == crate::project_git_info_value::project_git_info_value()
                .commit()
                .as_ref(),
    )
}
