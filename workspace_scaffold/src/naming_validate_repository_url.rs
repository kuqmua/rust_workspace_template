#[allow(
    clippy::single_call_fn,
    reason = "the named URL validator has a focused unit test and is the CLI validation owner"
)]
pub(crate) fn naming_validate_repository_url(
    repository_url_ref: crate::repository_url_ref::RepositoryUrlRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let Some(after_scheme) = repository_url_ref
        .get()
        .strip_prefix(constants_str::HTTPS_SCHEME_PREFIX)
    else {
        return Err(crate::scaffold_error::ScaffoldError::RepositoryUrl);
    };
    let has_host = after_scheme
        .split(['/', '?', '#'])
        .next()
        .is_some_and(|host| !host.is_empty());
    if !has_host || repository_url_ref.get().ends_with('/') {
        return Err(crate::scaffold_error::ScaffoldError::RepositoryUrl);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_repository_url_requires_host() {
        let hostless_path = format!(
            "{}{}{}",
            constants_str::HTTPS_SCHEME_PREFIX,
            constants_str::SLASH,
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE
        );
        assert!(
            [constants_str::HTTPS_SCHEME_PREFIX, hostless_path.as_str()]
                .into_iter()
                .all(|url| matches!(
                    crate::naming_validate_repository_url::naming_validate_repository_url(
                        crate::repository_url_ref::RepositoryUrlRef::from(url)
                    ),
                    Err(crate::scaffold_error::ScaffoldError::RepositoryUrl)
                ))
        );
    }
}
