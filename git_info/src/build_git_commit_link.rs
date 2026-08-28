use crate::domain_types::{GitCommitIdRef, GitCommitLink, build_git_commit_link_cow};

#[must_use]
pub fn build_git_commit_link<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> GitCommitLink
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    build_git_commit_link_cow(commit_id).into()
}
