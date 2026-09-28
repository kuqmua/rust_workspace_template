pub fn build_git_commit_link<'commit_lt, CommitIdTy>(
    commit_id_ty: CommitIdTy,
) -> Result<
    crate::git_commit_link::GitCommitLink,
    crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
>
where
    CommitIdTy: Into<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>,
{
    crate::build_git_commit_link_cow::build_git_commit_link_cow(commit_id_ty)
        .map(crate::git_commit_link::GitCommitLink::from)
}
