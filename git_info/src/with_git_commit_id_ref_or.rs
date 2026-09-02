pub(crate) fn with_git_commit_id_ref_or<'src, T, R>(
    t: &'src T,
    on_ref: impl FnOnce(crate::git_commit_id_ref::GitCommitIdRef<'src>) -> R,
    on_owned: impl FnOnce(&'src T) -> R,
) -> R
where
    T: ?Sized + crate::git_commit_id_provider::GitCommitIdProvider,
{
    t.git_commit_id_ref().map_or_else(|| on_owned(t), on_ref)
}
