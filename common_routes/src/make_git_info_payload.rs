use super::GitInfo;

#[cfg(test)]
pub(crate) const fn make_git_info_payload(commit: git_info::GitCommitLinkCow) -> GitInfo {
    GitInfo { commit }
}
