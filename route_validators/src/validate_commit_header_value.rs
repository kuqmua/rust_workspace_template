pub(crate) fn validate_commit_header_value(
    commit: crate::header_str_ref::HeaderStrRef<'_>,
) -> Result<(), crate::commit_error::CommitError> {
    git_info::validate_project_commit::validate_project_commit(commit.as_ref())
        .map_err(|error| {
            crate::commit_to_use::CommitToUse::from(<&'static str>::from(
                git_info::project_git_commit_link_ref::ProjectGitCommitLinkRef::from(error),
            ))
        })
        .map_err(
            |commit_to_use| crate::commit_error::CommitError::CommitNotEq {
                commit_not_eq: crate::commit_not_eq_message::CommitNotEqMessage::from(
                    constants_str::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
                ),
                commit_to_use,
                location: proc_macro_location_bang::location!(),
            },
        )
}
