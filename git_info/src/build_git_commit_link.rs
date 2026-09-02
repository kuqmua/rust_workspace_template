#[must_use]
pub fn build_git_commit_link<'commit_lt, CommitIdTy>(
    commit_id_ty: CommitIdTy,
) -> crate::git_commit_link::GitCommitLink
where
    CommitIdTy: Into<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>,
{
    crate::build_git_commit_link_cow::build_git_commit_link_cow(commit_id_ty).into()
}
