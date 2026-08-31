#[cfg(test)]
mod tests {
    #[test]
    fn accepts_explicit_loopback_test_databases() {
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
    fn rejects_ambiguous_and_non_loopback_targets_without_leaking_credentials() {
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
}
