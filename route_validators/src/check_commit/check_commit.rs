pub fn check_commit(
    enable_api_git_commit_check: super::EnableApiGitCommitCheck,
    headers: crate::domain_types::header_value::AxumHeadersRef<'_>,
) -> Result<(), super::CommitError> {
    if !enable_api_git_commit_check.0 {
        return Ok(());
    }
    super::validate_commit_header(headers)
}
