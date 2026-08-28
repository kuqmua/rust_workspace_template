use super::GitInfo;

#[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between endpoints and tests
pub(crate) const fn make_git_info_payload(
    commit: git_info::domain_types::GitCommitLinkCow,
) -> GitInfo {
    GitInfo { commit }
}
