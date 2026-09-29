#[cfg(test)]
mod tests {
    #[test]
    fn test_accepts_explicit_loopback_test_databases() {
        let all_accepted = [
            constants_str::POSTGRES_USER_SECRET_LOCALHOST_TEST,
            constants_str::POSTGRESQL_USER_SECRET_127_0_0_1_5432_APP_TEST_QUESTION_SSLMODE,
            constants_str::POSTGRES_USER_SECRET_PATH_1_TEST_CI_FRAGMENT,
        ]
        .into_iter()
        .all(|url| {
            crate::validate_test_database_url::validate_test_database_url(
                crate::url_ref::UrlRef::from(url),
            )
            .is_ok()
        });
        assert!(all_accepted);
    }
    #[test]
    fn test_rejects_ambiguous_and_non_loopback_targets_without_leaking_credentials() {
        let all_rejected_without_credentials = [
            constants_str::POSTGRES_ADMIN_PRODUCTION_SECRET_DB_EXAMPLE_COM_APP_TEST,
            constants_str::POSTGRES_ADMIN_PRODUCTION_SECRET_LOCALHOST_POSTGRES,
            constants_str::POSTGRES_ADMIN_PRODUCTION_SECRET_LOCALHOST_PRODUCTION,
            constants_str::NOT_A_URL,
        ]
        .into_iter()
        .all(|url| {
            crate::validate_test_database_url::validate_test_database_url(
                crate::url_ref::UrlRef::from(url),
            )
            .is_err_and(|error| {
                let message = error.to_string();
                !message.contains(constants_str::ADMIN_ALT)
                    && !message.contains(constants_str::PRODUCTION_SECRET)
            })
        });
        assert!(all_rejected_without_credentials);
    }
    #[test]
    fn test_rejects_query_parameters_that_override_checked_database_target() {
        let base = constants_str::POSTGRES_USER_SECRET_LOCALHOST_TEST;
        let overrides = [
            constants_str::DATABASE_QUERY_HOST_OVERRIDE,
            constants_str::DATABASE_QUERY_HOSTADDR_OVERRIDE,
            constants_str::DATABASE_QUERY_DBNAME_OVERRIDE,
            constants_str::DATABASE_QUERY_ENCODED_HOST_OVERRIDE,
        ];
        assert!(overrides.into_iter().all(|suffix| {
            let url = format!("{base}{suffix}");
            matches!(
                crate::validate_test_database_url::validate_test_database_url(
                    crate::url_ref::UrlRef::from(url.as_str()),
                ),
                Err(crate::url_error::UrlError::Malformed)
            )
        }));
    }
}
