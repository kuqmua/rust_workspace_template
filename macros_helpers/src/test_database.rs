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
pub enum UrlEr {
    #[error("database name is not explicitly test-only: {target}")]
    AmbiguousDatabase { target: SanitizedDatabaseTarget },
    #[error("test database URL is malformed")]
    Malformed,
    #[error("test database host is not loopback: {target}")]
    NonLoopback { target: SanitizedDatabaseTarget },
}
pub fn validate_test_database_url(url: UrlRef<'_>) -> Result<SanitizedDatabaseTarget, UrlEr> {
    let Some((scheme, after_scheme)) = url.0.split_once("://") else {
        return Err(UrlEr::Malformed);
    };
    if !matches!(scheme, "postgres" | "postgresql") {
        return Err(UrlEr::Malformed);
    }
    let Some((authority, path_and_suffix)) = after_scheme.split_once('/') else {
        return Err(UrlEr::Malformed);
    };
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, value)| value);
    let host = if let Some(without_opening_bracket) = host_port.strip_prefix('[') {
        let Some((value, suffix)) = without_opening_bracket.split_once(']') else {
            return Err(UrlEr::Malformed);
        };
        if !suffix.is_empty() && !suffix.starts_with(':') {
            return Err(UrlEr::Malformed);
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
        .ok_or(UrlEr::Malformed)?;
    let target = SanitizedDatabaseTarget::try_from(format!("{scheme}://{host}/{database}"))
        .map_err(|_error| UrlEr::Malformed)?;
    if !matches!(host, "localhost" | "127.0.0.1" | "::1") {
        return Err(UrlEr::NonLoopback { target });
    }
    if database != "test" && !database.starts_with("test_") && !database.ends_with("_test") {
        return Err(UrlEr::AmbiguousDatabase { target });
    }
    Ok(target)
}
#[cfg(test)]
mod tests {
    #[test]
    fn accepts_explicit_loopback_test_databases() {
        let all_accepted = [
            "postgres://user:secret@localhost/test",
            "postgresql://user:secret@127.0.0.1:5432/app_test?sslmode=disable",
            "postgres://user:secret@[::1]/test_ci#fragment",
        ]
        .into_iter()
        .all(|url| super::validate_test_database_url(super::UrlRef::from(url)).is_ok());
        assert!(all_accepted);
    }
    #[test]
    fn rejects_ambiguous_and_non_loopback_targets_without_leaking_credentials() {
        let all_rejected_without_credentials = [
            "postgres://admin:production-secret@db.example.com/app_test",
            "postgres://admin:production-secret@localhost/postgres",
            "postgres://admin:production-secret@localhost/production",
            "not-a-url",
        ]
        .into_iter()
        .all(|url| {
            super::validate_test_database_url(super::UrlRef::from(url)).is_err_and(|error| {
                let message = error.to_string();
                !message.contains("admin") && !message.contains("production-secret")
            })
        });
        assert!(all_rejected_without_credentials);
    }
}
