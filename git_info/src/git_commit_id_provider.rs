pub trait GitCommitIdProvider {
    fn git_commit_id(
        &self,
    ) -> Result<
        crate::git_commit_id::GitCommitId,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    >;
    fn git_commit_id_cow(
        &self,
    ) -> Result<
        crate::git_commit_id_cow::GitCommitIdCow<'_>,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        crate::with_git_commit_id_ref_or::with_git_commit_id_ref_or(
            self,
            |commit_id| {
                crate::git_commit_id_cow::GitCommitIdCow::try_from(std::borrow::Cow::Borrowed(
                    <&str>::from(commit_id),
                ))
            },
            |src| {
                crate::git_commit_id_cow::GitCommitIdCow::try_from(std::borrow::Cow::Owned(
                    String::from(src.git_commit_id()?),
                ))
            },
        )
    }
    fn git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        git_commit_id_fallback: &'commit_id_lt mut crate::git_commit_id_fallback::GitCommitIdFallback,
    ) -> Result<
        crate::git_commit_id_ref::GitCommitIdRef<'commit_id_lt>,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        crate::with_git_commit_id_ref_or::with_git_commit_id_ref_or(
            self,
            |commit_id| {
                crate::git_commit_id_cow::GitCommitIdCow::try_from(std::borrow::Cow::Borrowed(
                    <&str>::from(commit_id),
                ))
                .map(|_validated_commit| commit_id)
            },
            |src| {
                let fallback_commit = match &mut **git_commit_id_fallback {
                    Some(commit) => commit,
                    slot @ None => slot.insert(src.git_commit_id()?),
                };
                Ok(crate::git_commit_id_ref::GitCommitIdRef::from(
                    AsRef::<str>::as_ref(&*fallback_commit),
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
    ) -> Result<R, crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError>
    {
        let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
        Ok(f(self.git_commit_id_or_else(&mut fallback)?))
    }
}
impl<T: ?Sized + AsRef<str>> GitCommitIdProvider for T {
    fn git_commit_id(
        &self,
    ) -> Result<
        crate::git_commit_id::GitCommitId,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        crate::git_commit_id::GitCommitId::try_from(crate::git_commit_id_ref::GitCommitIdRef::from(
            self.as_ref(),
        ))
    }
    fn git_commit_id_ref(&self) -> Option<crate::git_commit_id_ref::GitCommitIdRef<'_>> {
        Some(crate::git_commit_id_ref::GitCommitIdRef::from(
            self.as_ref(),
        ))
    }
}
