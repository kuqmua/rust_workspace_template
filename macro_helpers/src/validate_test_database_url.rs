pub fn validate_test_database_url(
    url: crate::url_ref::UrlRef<'_>,
) -> Result<crate::sanitized_database_target::SanitizedDatabaseTarget, crate::url_error::UrlError> {
    let Some((scheme, after_scheme)) = url.0.split_once(constants_str::catalog::TEXT_ALT_10) else {
        return Err(crate::url_error::UrlError::Malformed);
    };
    if !matches!(
        scheme,
        constants_str::integration_fixtures::POSTGRES
            | constants_str::integration_fixtures::POSTGRESQL
    ) {
        return Err(crate::url_error::UrlError::Malformed);
    }
    let Some((authority, path_and_suffix)) = after_scheme.split_once('/') else {
        return Err(crate::url_error::UrlError::Malformed);
    };
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, value)| value);
    let host = if let Some(without_opening_bracket) = host_port.strip_prefix('[') {
        let Some((value, suffix)) = without_opening_bracket.split_once(']') else {
            return Err(crate::url_error::UrlError::Malformed);
        };
        if !suffix.is_empty() && !suffix.starts_with(':') {
            return Err(crate::url_error::UrlError::Malformed);
        }
        value
    } else {
        host_port
            .split_once(':')
            .map_or(host_port, |(value, _)| value)
    };
    let database = path_and_suffix
        .split(['?', '#'])
        .next()
        .filter(|value| !value.is_empty())
        .ok_or(crate::url_error::UrlError::Malformed)?;
    let target = crate::sanitized_database_target::SanitizedDatabaseTarget::try_from(format!(
        "{scheme}://{host}/{database}"
    ))
    .map_err(|_error| crate::url_error::UrlError::Malformed)?;
    if !matches!(
        host,
        constants_str::integration_fixtures::LOCALHOST
            | constants_str::catalog::VALUE_127_0_0_1
            | constants_str::integration_fixtures::PATH_1
    ) {
        return Err(crate::url_error::UrlError::NonLoopback { target });
    }
    if database != constants_str::catalog::TEST_ALT_3
        && !database.starts_with(constants_str::catalog::TEST_ALT_4)
        && !database.ends_with(constants_str::catalog::TEST)
    {
        return Err(crate::url_error::UrlError::AmbiguousDatabase { target });
    }
    Ok(target)
}
