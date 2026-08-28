pub(crate) fn with_git_commit_id_ref_or<'src, T, R>(
    src: &'src T,
    on_ref: impl FnOnce(crate::git_commit_id_ref::GitCommitIdRef<'src>) -> R,
    on_owned: impl FnOnce(&'src T) -> R,
) -> R
where
    T: ?Sized + crate::git_commit_id_provider::GitCommitIdProvider,
{
    src.git_commit_id_ref()
        .map_or_else(|| on_owned(src), on_ref)
}
