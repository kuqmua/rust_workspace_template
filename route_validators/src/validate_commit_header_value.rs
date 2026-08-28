pub(crate) fn validate_commit_header_value(
    commit: crate::header_value::HeaderStrRef<'_>,
) -> Result<(), super::CommitError> {
    git_info::validate_project_commit(commit.as_ref())
        .map_err(|error| {
            super::CommitToUse::from(<&'static str>::from(
                git_info::ProjectGitCommitLinkRef::from(error),
            ))
        })
        .map_err(|commit_to_use| super::CommitError::CommitNotEq {
            commit_not_eq: super::CommitNotEqMessage::from(
                constants_str::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
            ),
            commit_to_use,
            location: location_macros::location!(),
        })
}
