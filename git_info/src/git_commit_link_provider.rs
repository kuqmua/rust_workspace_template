use super::{GitCommitIdProvider, GitCommitLink, GitCommitLinkCow, build_git_commit_link_cow};

pub trait GitCommitLinkProvider {
    fn build_git_commit_link(&self) -> GitCommitLink {
        self.build_git_commit_link_cow().into()
    }
    fn build_git_commit_link_cow(&self) -> GitCommitLinkCow;
}
impl<T: ?Sized + GitCommitIdProvider> GitCommitLinkProvider for T {
    fn build_git_commit_link_cow(&self) -> GitCommitLinkCow {
        self.with_git_commit_id(|commit_id| build_git_commit_link_cow(commit_id))
    }
}
