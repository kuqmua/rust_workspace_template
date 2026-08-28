use crate::{BASE_GIT_COMMIT_LINK_LEN, GitCommitIdRef, GitCommitLinkCapacity};

#[must_use]
pub fn git_commit_link_capacity_value<'commit_lt, CommitIdTy>(
    commit_id: CommitIdTy,
) -> GitCommitLinkCapacity
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    GitCommitLinkCapacity::from(BASE_GIT_COMMIT_LINK_LEN.saturating_add(commit_id_ref.0.len()))
}
