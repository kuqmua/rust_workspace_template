#[allow(clippy::single_call_fn)] // separates commit-value validation from header parsing for reuse and focused tests
pub(crate) fn validate_commit_header_value(
    commit: crate::domain_types::header_value::HeaderStrRef<'_>,
) -> Result<(), super::CommitError> {
    git_info::domain_types::validate_project_commit(commit.as_ref())
        .map_err(|error| {
            super::CommitToUse::from(<&'static str>::from(
                git_info::domain_types::ProjectGitCommitLinkRef::from(error),
            ))
        })
        .map_err(super::CommitError::commit_not_eq)
}
