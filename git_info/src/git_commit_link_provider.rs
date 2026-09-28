pub trait GitCommitLinkProvider {
    fn build_git_commit_link(
        &self,
    ) -> Result<
        crate::git_commit_link::GitCommitLink,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        self.build_git_commit_link_cow()
            .map(crate::git_commit_link::GitCommitLink::from)
    }
    fn build_git_commit_link_cow(
        &self,
    ) -> Result<
        crate::git_commit_link_cow::GitCommitLinkCow,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    >;
}
impl<T: ?Sized + crate::git_commit_id_provider::GitCommitIdProvider> GitCommitLinkProvider for T {
    fn build_git_commit_link_cow(
        &self,
    ) -> Result<
        crate::git_commit_link_cow::GitCommitLinkCow,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        self.with_git_commit_id(|commit_id| {
            crate::build_git_commit_link_cow::build_git_commit_link_cow(commit_id)
        })?
    }
}
