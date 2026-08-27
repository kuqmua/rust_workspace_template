use super::super::{GitCommitIdRef, GitCommitLink, git_commit_link_cow};

#[must_use]
pub fn git_commit_link<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> GitCommitLink
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    git_commit_link_cow(commit_id).into()
}
