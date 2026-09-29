pub fn validate_test_database_url(
    url_ref: crate::url_ref::UrlRef<'_>,
) -> Result<crate::sanitized_database_target::SanitizedDatabaseTarget, crate::url_error::UrlError> {
    let Some((scheme, after_scheme)) = url_ref.as_str().split_once(constants_str::TEXT_ALT_10)
    else {
        return Err(crate::url_error::UrlError::Malformed);
    };
    if !matches!(scheme, constants_str::POSTGRES | constants_str::POSTGRESQL) {
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
    let path_and_query = path_and_suffix
        .split_once('#')
        .map_or(path_and_suffix, |(value, _)| value);
    let (database, optional_query) = path_and_query
        .split_once('?')
        .map_or((path_and_query, None), |(value, query)| {
            (value, Some(query))
        });
    if optional_query.is_some_and(|query_text| {
        query_text.split('&').any(|parameter| {
            let key = parameter.split_once('=').map_or(parameter, |(key, _)| key);
            key.contains('%')
                || [
                    constants_str::DATABASE_QUERY_HOST_KEY,
                    constants_str::DATABASE_QUERY_HOSTADDR_KEY,
                    constants_str::DATABASE_QUERY_DBNAME_KEY,
                ]
                .into_iter()
                .any(|override_key| key.eq_ignore_ascii_case(override_key))
        })
    }) {
        return Err(crate::url_error::UrlError::Malformed);
    }
    if database.is_empty() {
        return Err(crate::url_error::UrlError::Malformed);
    }
    let target = crate::sanitized_database_target::SanitizedDatabaseTarget::try_from(format!(
        "{scheme}://{host}/{database}"
    ))
    .map_err(|_error| crate::url_error::UrlError::Malformed)?;
    if !matches!(
        host,
        constants_str::LOCALHOST | constants_str::VALUE_127_0_0_1 | constants_str::PATH_1
    ) {
        return Err(crate::url_error::UrlError::NonLoopback { target });
    }
    if database != constants_str::TEST_ALT_3
        && !database.starts_with(constants_str::TEST_ALT_4)
        && !database.ends_with(constants_str::TEST)
    {
        return Err(crate::url_error::UrlError::AmbiguousDatabase { target });
    }
    Ok(target)
}
