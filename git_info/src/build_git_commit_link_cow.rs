#[must_use]
pub fn build_git_commit_link_cow<'commit_lt, CommitIdTy>(
    commit_id: CommitIdTy,
) -> crate::git_commit_link_cow::GitCommitLinkCow
where
    CommitIdTy: Into<crate::git_commit_id_ref::GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    if commit_id_ref.as_ref().len()
        > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN
            .saturating_sub(crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN)
    {
        return crate::git_commit_link_cow::GitCommitLinkCow::try_from(
            std::borrow::Cow::Owned(
                crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
                    len: crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN
                        .saturating_add(commit_id_ref.as_ref().len()),
                    max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
                }
                .to_string(),
            ),
        )
        .unwrap_or_else(crate::git_commit_link_cow::GitCommitLinkCow::from);
    }
    if bool::from(crate::check_is_project_commit::check_is_project_commit(
        commit_id_ref,
    )) {
        return crate::git_commit_link_cow::GitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(
            <&str>::from(
                crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value(),
            ),
        ))
        .unwrap_or_else(crate::git_commit_link_cow::GitCommitLinkCow::from);
    }
    let cap = crate::git_commit_link_capacity_value::git_commit_link_capacity_value(commit_id_ref);
    let mut output = String::with_capacity(*cap);
    output.push_str(constants_str::NAMING_GITHUB_URL);
    output.push_str(constants_str::GIT_INFO_TREE_SEGMENT);
    output.push_str(commit_id_ref.as_ref());
    crate::git_commit_link_cow::GitCommitLinkCow::try_from(std::borrow::Cow::Owned(output))
        .unwrap_or_else(crate::git_commit_link_cow::GitCommitLinkCow::from)
}
