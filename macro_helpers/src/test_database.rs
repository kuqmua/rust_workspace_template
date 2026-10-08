#[cfg(test)]
mod tests {
    #[test]
    fn test_database_url_preserves_exact_sanitized_targets_and_error_precedence() {
        [constants_str::POSTGRES, constants_str::POSTGRESQL]
            .into_iter()
            .fold((), |(), scheme| {
                [
                    (constants_str::LOCALHOST, constants_str::TEST_ALT_3, 0u8),
                    (constants_str::PATH_1, constants_str::TEST_ALT_3, 0u8),
                    (constants_str::X, constants_str::POSTGRES, 1u8),
                    (constants_str::LOCALHOST, constants_str::POSTGRES, 2u8),
                ]
                .into_iter()
                .fold((), |(), (host, database, expected_variant)| {
                    let authority_host = if host == constants_str::PATH_1 {
                        format!("[{host}]")
                    } else {
                        host.to_owned()
                    };
                    let url = format!(
                        "{scheme}{}{}:{}@{authority_host}:5432/{database}?{}={}#{}",
                        constants_str::TEXT_ALT_10,
                        constants_str::ADMIN_ALT,
                        constants_str::PRODUCTION_SECRET,
                        constants_str::X,
                        constants_str::X,
                        constants_str::X,
                    );
                    let expected_target =
                        format!("{scheme}{}{host}/{database}", constants_str::TEXT_ALT_10);
                    let result = crate::validate_test_database_url::validate_test_database_url(
                        crate::url_ref::UrlRef::from(url.as_str()),
                    );
                    assert!(match (expected_variant, result) {
                        (0u8, Ok(target))
                        | (1u8, Err(crate::url_error::UrlError::NonLoopback { target }))
                        | (2u8, Err(crate::url_error::UrlError::AmbiguousDatabase { target })) => {
                            target.to_string() == expected_target
                        }
                        _ => false,
                    });
                });
            });
    }

    #[test]
    fn test_database_url_rejects_case_insensitive_and_bare_query_overrides() {
        let base = constants_str::POSTGRES_USER_SECRET_LOCALHOST_TEST;
        [
            constants_str::DATABASE_QUERY_HOST_KEY,
            constants_str::DATABASE_QUERY_HOSTADDR_KEY,
            constants_str::DATABASE_QUERY_DBNAME_KEY,
        ]
        .into_iter()
        .fold((), |(), key| {
            let uppercase = key.to_ascii_uppercase();
            [
                format!("{base}?{uppercase}"),
                format!("{base}?{uppercase}={}", constants_str::X),
                format!(
                    "{base}?{}={}&{uppercase}=",
                    constants_str::X,
                    constants_str::X
                ),
            ]
            .into_iter()
            .fold((), |(), url| {
                assert_eq!(
                    crate::validate_test_database_url::validate_test_database_url(
                        crate::url_ref::UrlRef::from(url.as_str()),
                    ),
                    Err(crate::url_error::UrlError::Malformed)
                );
            });
        });
    }
    #[test]
    fn test_database_url_rejects_malformed_scheme_authority_and_empty_database() {
        let separator = constants_str::TEXT_ALT_10;
        [
            format!(
                "{}{}{}{}{}",
                constants_str::HTTP,
                separator,
                constants_str::LOCALHOST,
                '/',
                constants_str::TEST_ALT_3
            ),
            format!(
                "{}{}{}",
                constants_str::POSTGRES,
                separator,
                constants_str::LOCALHOST
            ),
            format!(
                "{}{}{}{}",
                constants_str::POSTGRES,
                separator,
                constants_str::LOCALHOST,
                '/'
            ),
            format!(
                "{}{}{}{}{}{}",
                constants_str::POSTGRES,
                separator,
                '[',
                constants_str::PATH_1,
                '/',
                constants_str::TEST_ALT_3
            ),
            format!(
                "{}{}{}{}{}{}{}{}",
                constants_str::POSTGRES,
                separator,
                '[',
                constants_str::PATH_1,
                ']',
                constants_str::X,
                '/',
                constants_str::TEST_ALT_3
            ),
        ]
        .into_iter()
        .fold((), |(), url| {
            assert_eq!(
                crate::validate_test_database_url::validate_test_database_url(
                    crate::url_ref::UrlRef::from(url.as_str())
                ),
                Err(crate::url_error::UrlError::Malformed)
            );
        });
    }

    #[test]
    fn test_database_url_sanitized_target_preserves_exact_storage_boundary() {
        let base = format!(
            "{}{}{}{}",
            constants_str::POSTGRES,
            constants_str::TEXT_ALT_10,
            constants_str::LOCALHOST,
            '/'
        );
        let prefix = constants_str::TEST_ALT_4;
        let maximum = format!(
            "{}{}{}",
            base,
            prefix,
            constants_str::X.repeat(constants_usize::VALUE_4_096 - base.len() - prefix.len())
        );
        assert_eq!(maximum.len(), constants_usize::VALUE_4_096);
        assert!(
            crate::validate_test_database_url::validate_test_database_url(
                crate::url_ref::UrlRef::from(maximum.as_str())
            )
            .is_ok_and(|target| target.to_string() == maximum)
        );
        let oversized = format!("{}{}", maximum, constants_str::X);
        assert_eq!(
            crate::validate_test_database_url::validate_test_database_url(
                crate::url_ref::UrlRef::from(oversized.as_str())
            ),
            Err(crate::url_error::UrlError::Malformed)
        );
    }
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
