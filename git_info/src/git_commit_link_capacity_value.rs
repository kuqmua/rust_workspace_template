#[must_use]
pub fn git_commit_link_capacity_value<'commit_lt, CommitIdTy>(
    commit_id_ty: CommitIdTy,
) -> crate::git_commit_link_capacity::GitCommitLinkCapacity
where
    CommitIdTy: Into<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id_ty.into();
    crate::git_commit_link_capacity::GitCommitLinkCapacity::from(
        crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN
            .saturating_add(commit_id_ref.as_ref().len()),
    )
}
