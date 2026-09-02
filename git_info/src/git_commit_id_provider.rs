pub trait GitCommitIdProvider {
    fn git_commit_id(&self) -> crate::git_commit_id::GitCommitId;
    fn git_commit_id_cow(&self) -> crate::git_commit_id_cow::GitCommitIdCow<'_> {
        crate::with_git_commit_id_ref_or::with_git_commit_id_ref_or(
            self,
            |commit_id| {
                crate::git_commit_id_cow::GitCommitIdCow::try_from(std::borrow::Cow::Borrowed(
                    <&str>::from(commit_id),
                ))
                .unwrap_or_else(crate::git_commit_id_cow::GitCommitIdCow::from)
            },
            |src| {
                crate::git_commit_id_cow::GitCommitIdCow::try_from(std::borrow::Cow::Owned(
                    String::from(src.git_commit_id()),
                ))
                .unwrap_or_else(crate::git_commit_id_cow::GitCommitIdCow::from)
            },
        )
    }
    fn git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        git_commit_id_fallback: &'commit_id_lt mut crate::git_commit_id_fallback::GitCommitIdFallback,
    ) -> crate::git_commit_id_ref::GitCommitIdRef<'commit_id_lt> {
        crate::with_git_commit_id_ref_or::with_git_commit_id_ref_or(
            self,
            |commit_id| commit_id,
            |src| {
                let fallback_commit =
                    git_commit_id_fallback.get_or_insert_with(|| src.git_commit_id());
                crate::git_commit_id_ref::GitCommitIdRef::from(AsRef::<str>::as_ref(
                    &*fallback_commit,
                ))
            },
        )
    }
    fn git_commit_id_ref(&self) -> Option<crate::git_commit_id_ref::GitCommitIdRef<'_>> {
        None
    }
    fn with_git_commit_id<R>(
        &self,
        f: impl FnOnce(crate::git_commit_id_ref::GitCommitIdRef<'_>) -> R,
    ) -> R {
        let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
        f(self.git_commit_id_or_else(&mut fallback))
    }
}
impl<T: ?Sized + AsRef<str>> GitCommitIdProvider for T {
    fn git_commit_id(&self) -> crate::git_commit_id::GitCommitId {
        crate::git_commit_id::GitCommitId::try_from(self.as_ref().to_owned())
            .unwrap_or_else(crate::git_commit_id::GitCommitId::from)
    }
    fn git_commit_id_ref(&self) -> Option<crate::git_commit_id_ref::GitCommitIdRef<'_>> {
        Some(crate::git_commit_id_ref::GitCommitIdRef::from(
            self.as_ref(),
        ))
    }
}
