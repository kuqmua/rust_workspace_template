const SANITIZED_DATABASE_TARGET_MAX_LEN: usize = 4096;
#[derive(Clone, Copy, Debug)]
pub struct UrlRef<'url_lt>(&'url_lt str);
impl<'url_lt> From<&'url_lt str> for UrlRef<'url_lt> {
    fn from(value: &'url_lt str) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, newtype::BoundedString)]
#[bounded_string(max = SANITIZED_DATABASE_TARGET_MAX_LEN)]
pub struct SanitizedDatabaseTarget(String);
impl std::fmt::Display for SanitizedDatabaseTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UrlError {
    #[error("database name is not explicitly test-only: {target}")]
    AmbiguousDatabase { target: SanitizedDatabaseTarget },
    #[error("test database URL is malformed")]
    Malformed,
    #[error("test database host is not loopback: {target}")]
    NonLoopback { target: SanitizedDatabaseTarget },
}
pub fn validate_test_database_url(url: UrlRef<'_>) -> Result<SanitizedDatabaseTarget, UrlError> {
    let Some((scheme, after_scheme)) = url.0.split_once(str_constants::TEXT_ALT_10) else {
        return Err(UrlError::Malformed);
    };
    if !matches!(scheme, str_constants::POSTGRES | str_constants::POSTGRESQL) {
        return Err(UrlError::Malformed);
    }
    let Some((authority, path_and_suffix)) = after_scheme.split_once('/') else {
        return Err(UrlError::Malformed);
    };
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, value)| value);
    let host = if let Some(without_opening_bracket) = host_port.strip_prefix('[') {
        let Some((value, suffix)) = without_opening_bracket.split_once(']') else {
            return Err(UrlError::Malformed);
        };
        if !suffix.is_empty() && !suffix.starts_with(':') {
            return Err(UrlError::Malformed);
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
        .ok_or(UrlError::Malformed)?;
    let target = SanitizedDatabaseTarget::try_from(format!("{scheme}://{host}/{database}"))
        .map_err(|_error| UrlError::Malformed)?;
    if !matches!(
        host,
        str_constants::LOCALHOST | str_constants::VALUE_127_0_0_1 | str_constants::PATH_1
    ) {
        return Err(UrlError::NonLoopback { target });
    }
    if database != str_constants::TEST_ALT_3
        && !database.starts_with(str_constants::TEST_ALT_4)
        && !database.ends_with(str_constants::TEST)
    {
        return Err(UrlError::AmbiguousDatabase { target });
    }
    Ok(target)
}
#[cfg(test)]
mod tests {
    #[test]
    fn accepts_explicit_loopback_test_databases() {
        let all_accepted = [
            str_constants::POSTGRES_USER_SECRET_LOCALHOST_TEST,
            str_constants::POSTGRESQL_USER_SECRET_127_0_0_1_5432_APP_TEST_QUESTION_SSLMODE,
            str_constants::POSTGRES_USER_SECRET_PATH_1_TEST_CI_FRAGMENT,
        ]
        .into_iter()
        .all(|url| super::validate_test_database_url(super::UrlRef::from(url)).is_ok());
        assert!(all_accepted);
    }
    #[test]
    fn rejects_ambiguous_and_non_loopback_targets_without_leaking_credentials() {
        let all_rejected_without_credentials = [
            str_constants::POSTGRES_ADMIN_PRODUCTION_SECRET_DB_EXAMPLE_COM_APP_TEST,
            str_constants::POSTGRES_ADMIN_PRODUCTION_SECRET_LOCALHOST_POSTGRES,
            str_constants::POSTGRES_ADMIN_PRODUCTION_SECRET_LOCALHOST_PRODUCTION,
            str_constants::NOT_A_URL,
        ]
        .into_iter()
        .all(|url| {
            super::validate_test_database_url(super::UrlRef::from(url)).is_err_and(|error| {
                let message = error.to_string();
                !message.contains(str_constants::ADMIN_ALT)
                    && !message.contains(str_constants::PRODUCTION_SECRET)
            })
        });
        assert!(all_rejected_without_credentials);
    }
}
