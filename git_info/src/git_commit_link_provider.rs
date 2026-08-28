pub trait GitCommitLinkProvider {
    fn build_git_commit_link(&self) -> crate::git_commit_link::GitCommitLink {
        self.build_git_commit_link_cow().into()
    }
    fn build_git_commit_link_cow(&self) -> crate::git_commit_link_cow::GitCommitLinkCow;
}
impl<T: ?Sized + crate::git_commit_id_provider::GitCommitIdProvider> GitCommitLinkProvider for T {
    fn build_git_commit_link_cow(&self) -> crate::git_commit_link_cow::GitCommitLinkCow {
        self.with_git_commit_id(|commit_id| {
            crate::build_git_commit_link_cow::build_git_commit_link_cow(commit_id)
        })
    }
}
