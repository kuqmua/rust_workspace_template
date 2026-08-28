use crate::{
    GitCommitId, GitCommitIdCow, GitCommitIdFallback, GitCommitIdRef, with_git_commit_id_ref_or,
};

pub trait GitCommitIdProvider {
    fn git_commit_id(&self) -> GitCommitId;
    fn git_commit_id_cow(&self) -> GitCommitIdCow<'_> {
        with_git_commit_id_ref_or(
            self,
            |commit_id| {
                GitCommitIdCow::try_from(std::borrow::Cow::Borrowed(commit_id.0))
                    .unwrap_or_else(GitCommitIdCow::from)
            },
            |src| {
                GitCommitIdCow::try_from(std::borrow::Cow::Owned(src.git_commit_id().0))
                    .unwrap_or_else(GitCommitIdCow::from)
            },
        )
    }
    fn git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        fallback: &'commit_id_lt mut GitCommitIdFallback,
    ) -> GitCommitIdRef<'commit_id_lt> {
        with_git_commit_id_ref_or(
            self,
            |commit_id| commit_id,
            |src| {
                fallback
                    .0
                    .get_or_insert_with(|| src.git_commit_id())
                    .0
                    .as_str()
                    .into()
            },
        )
    }
    fn git_commit_id_ref(&self) -> Option<GitCommitIdRef<'_>> {
        None
    }
    fn with_git_commit_id<R>(&self, f: impl FnOnce(GitCommitIdRef<'_>) -> R) -> R {
        let mut fallback = GitCommitIdFallback::from(None);
        f(self.git_commit_id_or_else(&mut fallback))
    }
}
impl<T: ?Sized + AsRef<str>> GitCommitIdProvider for T {
    fn git_commit_id(&self) -> GitCommitId {
        GitCommitId::try_from(self.as_ref().to_owned()).unwrap_or_else(GitCommitId::from)
    }
    fn git_commit_id_ref(&self) -> Option<GitCommitIdRef<'_>> {
        Some(GitCommitIdRef::from(self.as_ref()))
    }
}
