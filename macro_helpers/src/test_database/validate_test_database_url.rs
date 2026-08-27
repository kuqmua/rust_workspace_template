pub fn validate_test_database_url(
    url: super::UrlRef<'_>,
) -> Result<super::SanitizedDatabaseTarget, super::UrlError> {
    let Some((scheme, after_scheme)) = url.0.split_once(constants_str::TEXT_ALT_10) else {
        return Err(super::UrlError::Malformed);
    };
    if !matches!(scheme, constants_str::POSTGRES | constants_str::POSTGRESQL) {
        return Err(super::UrlError::Malformed);
    }
    let Some((authority, path_and_suffix)) = after_scheme.split_once('/') else {
        return Err(super::UrlError::Malformed);
    };
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, value)| value);
    let host = if let Some(without_opening_bracket) = host_port.strip_prefix('[') {
        let Some((value, suffix)) = without_opening_bracket.split_once(']') else {
            return Err(super::UrlError::Malformed);
        };
        if !suffix.is_empty() && !suffix.starts_with(':') {
            return Err(super::UrlError::Malformed);
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
        .ok_or(super::UrlError::Malformed)?;
    let target = super::SanitizedDatabaseTarget::try_from(format!("{scheme}://{host}/{database}"))
        .map_err(|_error| super::UrlError::Malformed)?;
    if !matches!(
        host,
        constants_str::LOCALHOST | constants_str::VALUE_127_0_0_1 | constants_str::PATH_1
    ) {
        return Err(super::UrlError::NonLoopback { target });
    }
    if database != constants_str::TEST_ALT_3
        && !database.starts_with(constants_str::TEST_ALT_4)
        && !database.ends_with(constants_str::TEST)
    {
        return Err(super::UrlError::AmbiguousDatabase { target });
    }
    Ok(target)
}
