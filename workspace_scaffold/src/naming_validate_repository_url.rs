pub(crate) fn naming_validate_repository_url(
    repository_url_ref: crate::repository_url_ref::RepositoryUrlRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    if !repository_url_ref
        .get()
        .starts_with(constants_str::HTTPS_SCHEME_PREFIX)
        || repository_url_ref.get().ends_with('/')
    {
        return Err(crate::scaffold_error::ScaffoldError::RepositoryUrl);
    }
    Ok(())
}
