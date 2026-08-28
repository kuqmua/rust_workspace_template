#[allow(clippy::single_call_fn)] // keeps repository URL validation separate from CLI orchestration
pub(crate) fn naming_validate_repository_url(
    value: super::RepositoryUrlRef<'_>,
) -> Result<(), super::ScaffoldError> {
    if !value.0.starts_with(constants_str::HTTPS_SCHEME_PREFIX) || value.0.ends_with('/') {
        return Err(super::ScaffoldError::RepositoryUrl);
    }
    Ok(())
}
