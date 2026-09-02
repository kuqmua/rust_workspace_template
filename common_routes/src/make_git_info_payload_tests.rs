#[cfg(test)]
pub(crate) const fn make_git_info_payload(
    git_commit_link_cow: git_info::git_commit_link_cow::GitCommitLinkCow,
) -> crate::git_info::GitInfo {
    crate::git_info::GitInfo::from_commit(git_commit_link_cow)
}
