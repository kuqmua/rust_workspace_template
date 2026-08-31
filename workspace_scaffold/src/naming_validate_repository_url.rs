pub(crate) fn naming_validate_repository_url(
    value: crate::repository_url_ref::RepositoryUrlRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    if !value.get().starts_with(constants_str::HTTPS_SCHEME_PREFIX) || value.get().ends_with('/') {
        return Err(crate::scaffold_error::ScaffoldError::RepositoryUrl);
    }
    Ok(())
}
