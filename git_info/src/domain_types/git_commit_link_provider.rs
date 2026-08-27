use super::{GitCommitIdProvider, GitCommitLink, GitCommitLinkCow, git_commit_link_cow};

pub trait GitCommitLinkProvider {
    fn git_commit_link(&self) -> GitCommitLink {
        self.git_commit_link_cow().into()
    }
    fn git_commit_link_cow(&self) -> GitCommitLinkCow;
}
impl<T: ?Sized + GitCommitIdProvider> GitCommitLinkProvider for T {
    fn git_commit_link_cow(&self) -> GitCommitLinkCow {
        self.with_git_commit_id(|commit_id| git_commit_link_cow(commit_id))
    }
}
