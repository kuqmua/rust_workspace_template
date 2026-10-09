#[test]
fn test_authorization_rule_collection_exact_limit_preserves_order_and_rejects_overflow() {
    let rules = [
        server_admin_contract::admin_rule::AdminRule::MetricsRead,
        server_admin_contract::admin_rule::AdminRule::UsersRead,
    ];
    assert!(
        [0usize, 9999usize, 10000usize, 10001usize]
            .into_iter()
            .all(|count| {
                let input = rules.into_iter().cycle().take(count).collect::<Vec<_>>();
                let result = crate::admin_auth_rules::AdminAuthRules::try_from(input);
                if count == 10001usize {
                    matches!(
                        result,
                        Err(crate::admin_auth_collection_error::AdminAuthCollectionError::TooLarge)
                    )
                } else {
                    result.is_ok_and(|stored| {
                        stored.as_ref().len() == count
                            && stored
                                .as_ref()
                                .iter()
                                .zip(rules.into_iter().cycle())
                                .all(|(actual, expected)| *actual == expected)
                    })
                }
            })
    );
}

#[test]
fn test_authorization_role_collection_exact_limit_preserves_order_and_rejects_overflow() {
    let names = [constants_str::USER, constants_str::LOGIN];
    assert!([0usize, 9999usize, 10000usize, 10001usize].into_iter().all(|count| {
        names.into_iter().cycle().take(count)
            .map(|name| server_admin_contract::admin_role_name::AdminRoleName::try_from(name.to_owned()))
            .collect::<Result<Vec<_>, _>>().is_ok_and(|input| {
                let result = crate::runtime_admin_role_names::RuntimeAdminRoleNames::try_from(input);
                if count == 10001usize {
                    matches!(result, Err(crate::admin_auth_collection_error::AdminAuthCollectionError::TooLarge))
                } else {
                    result.is_ok_and(|stored| stored.as_ref().len() == count
                        && stored.as_ref().iter().zip(names.into_iter().cycle()).all(|(actual, expected)| actual.as_ref() == expected))
                }
            })
    }));
}

#[test]
fn test_page_total_conversion_numeric_endpoints_preserve_distinct_error_domains() {
    assert!(
        [i64::MIN, -1i64, 0i64, 1i64, i64::MAX]
            .into_iter()
            .all(|value| {
                let count = crate::admin_page_total_count::AdminPageTotalCount::from(value);
                let repository_result = crate::repository_page_total::repository_page_total(count);
                let application_result = crate::page_total::page_total(count);
                if let Ok(expected) = u64::try_from(value) {
                    repository_result.is_ok_and(|total| u64::from(total) == expected)
                        && application_result.is_ok_and(|total| u64::from(total) == expected)
                } else {
                    matches!(
                        repository_result,
                        Err(
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                        )
                    ) && matches!(
                        application_result,
                        Err(crate::admin_error::AdminError::Validation)
                    )
                }
            })
    );
}

#[test]
fn test_cleanup_report_total_includes_every_resource_and_saturates() {
    assert!(
        [
            ([0u64; 6usize], 0u64),
            ([u64::MAX; 6usize], u64::MAX),
            ([u64::MAX - 1u64, 1u64, 1u64, 0u64, 0u64, 0u64], u64::MAX),
        ]
        .into_iter()
        .chain((0usize..6usize).map(|position| {
            (
                std::array::from_fn(|index| if index == position { u64::MAX } else { 0u64 }),
                u64::MAX,
            )
        }))
        .all(|(values, expected)| {
            let [
                access_sessions,
                audit_log,
                idempotency,
                login_attempts,
                rate_limits,
                refresh_tokens,
            ] = values.map(crate::admin_cleanup_rows::AdminCleanupRows::from);
            crate::admin_cleanup_report::AdminCleanupReport::new(
                access_sessions,
                audit_log,
                idempotency,
                login_attempts,
                rate_limits,
                refresh_tokens,
            )
            .total_rows()
                == crate::admin_cleanup_rows::AdminCleanupRows::from(expected)
        })
    );
}

#[test]
fn test_cleanup_configuration_numeric_endpoints_preserve_values() {
    assert!([i64::MIN, -1i64, 0i64, 1i64, 9999i64, 10000i64, 10001i64, i64::MAX]
        .into_iter().all(|value| {
            let result = crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(value);
            if (1i64..=10000i64).contains(&value) {
                result.is_ok_and(|batch| *batch.get_inner() == value)
            } else {
                result == Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::BatchSizeOutOfRange)
            }
        }));
    assert!([i64::MIN, -1i64, 0i64, 1i64, i64::MAX].into_iter().all(|value| {
        let result = crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(value);
        if value > 0i64 {
            result.is_ok_and(|retention| retention.get() == value)
        } else {
            result == Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::RetentionMustBePositive)
        }
    }));
}

#[test]
fn test_cleanup_row_addition_preserves_values_and_saturates() {
    assert!(
        [
            (0u64, 0u64, 0u64),
            (0u64, u64::MAX, u64::MAX),
            (u64::MAX, 0u64, u64::MAX),
            (7u64, 11u64, 18u64),
            (u64::MAX - 1u64, 1u64, u64::MAX),
            (u64::MAX - 1u64, 2u64, u64::MAX),
            (u64::MAX, u64::MAX, u64::MAX)
        ]
        .into_iter()
        .all(|(left, right, expected)| {
            let left_rows = crate::admin_cleanup_rows::AdminCleanupRows::from(left);
            let right_rows = crate::admin_cleanup_rows::AdminCleanupRows::from(right);
            *(left_rows + right_rows).get_inner() == expected
                && *left_rows.saturating_add(right_rows).get_inner() == expected
        })
    );
}

#[test]
fn test_cleanup_configuration_preserves_distinct_retention_fields() {
    let retention = |value| {
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(value)
    };
    assert!(match (
        crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(1i64),
        retention(2i64),
        retention(3i64),
        retention(4i64),
        retention(5i64),
        retention(6i64),
    ) {
        (Ok(batch_size), Ok(auth), Ok(audit), Ok(rate_limit), Ok(completed), Ok(pending)) => {
            let configuration = crate::admin_cleanup_configuration::AdminCleanupConfiguration::new(
                batch_size, auth, audit, rate_limit, completed, pending,
            );
            *configuration.batch_size().get_inner() == 1i64
                && configuration.auth_retention().get() == 2i64
                && configuration.audit_retention().get() == 3i64
                && configuration.rate_limit_retention().get() == 4i64
                && configuration.idempotency_completed_retention().get() == 5i64
                && configuration.idempotency_pending_retention().get() == 6i64
        }
        _ => false,
    });
}

#[test]
fn test_cleanup_configuration_enforces_positive_bounded_values() {
    assert_eq!(
        crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
        Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::BatchSizeOutOfRange)
    );
    assert_eq!(
        crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(10_001i64),
        Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::BatchSizeOutOfRange)
    );
    assert_eq!(
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(
            constants_i64::ZERO
        ),
        Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::RetentionMustBePositive)
    );
    assert_eq!(
        crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(1_000i64).map(|_value| ()),
        Ok(())
    );
    assert_eq!(
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(3_600i64)
            .map(|_value| ()),
        Ok(())
    );
}
fn admin_secret(str: &str) -> server_admin_core::secrecy_admin_string::SecrecyAdminString {
    server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(str.to_owned())
        .expect(constants_str::DIAGNOSTIC_C2116874)
}
fn password(str: &str) -> crate::runtime_admin_password::RuntimeAdminPassword {
    crate::runtime_admin_password::RuntimeAdminPassword::new(admin_secret(str))
}
#[test]
fn test_rule_round_trip_is_exhaustive() {
    server_admin_contract::admin_rule::AdminRule::ALL
        .into_iter()
        .for_each(|rule| {
            assert_eq!(
                server_admin_contract::admin_rule::AdminRule::try_from(rule.as_str().as_ref())
                    .expect(constants_str::DIAGNOSTIC_0F53B75C),
                rule
            );
        });
}
#[test]
fn test_rule_serializes_as_public_contract_value() {
    assert_eq!(
        serde_json::to_string(&server_admin_contract::admin_rule::AdminRule::UsersRead)
            .expect(constants_str::DIAGNOSTIC_9A6B413E),
        constants_str::VALUE_6E7831EE
    );
}
#[test]
fn test_unknown_rule_is_rejected() {
    assert_eq!(
        server_admin_contract::admin_rule::AdminRule::try_from(constants_str::UNKNOWN_READ).err(),
        Some(server_admin_contract::admin_rule::AdminRuleTryFromStrError)
    );
}
#[test]
fn test_migration_inventory_is_not_empty() {
    let migrator = crate::migrator::migrator();
    let migrations = migrator.iter().collect::<Vec<_>>();
    assert_eq!(migrations.len(), 2usize);
    assert!(!migrator.ignore_missing);
    assert!(
        migrations
            .iter()
            .any(|migration| migration.description == constants_str::VALUE_6A6C872E)
    );
}
#[test]
fn test_rule_seed_contains_the_complete_typed_catalog() {
    let migration_sql = crate::migrator::migrator()
        .iter()
        .map(|migration| migration.sql.as_str())
        .collect::<Vec<_>>()
        .join(constants_str::EMPTY);
    assert!(
        server_admin_contract::admin_rule::AdminRule::ALL
            .into_iter()
            .all(|rule| {
                rule.as_str()
                    .as_ref()
                    .split_once(char::from(58u8))
                    .is_some_and(|(resource, action)| {
                        migration_sql.contains(resource) && migration_sql.contains(action)
                    })
            })
    );
}
#[tokio::test]
async fn test_password_hash_verifies_only_matching_password() {
    let hasher = crate::admin_password_hasher::AdminPasswordHasher::new(
        crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
            std::num::NonZeroUsize::new(1).expect(constants_str::DIAGNOSTIC_70761471),
        ),
    );
    let hash = hasher
        .hash(password(constants_str::CORRECT_PASSWORD_ALT))
        .await
        .expect(constants_str::DIAGNOSTIC_174A5D2F);
    assert!(
        hasher
            .verify(password(constants_str::CORRECT_PASSWORD_ALT), hash)
            .await
            .expect(constants_str::DIAGNOSTIC_604F40BE)
            .get()
    );
    let other_hash = hasher
        .hash(password(constants_str::CORRECT_PASSWORD_ALT))
        .await
        .expect(constants_str::DIAGNOSTIC_38819B94);
    assert!(
        !hasher
            .verify(password(constants_str::VALUE_3DFF7367), other_hash)
            .await
            .expect(constants_str::DIAGNOSTIC_ED6B499A)
            .get()
    );
}
#[test]
fn test_secrets_are_redacted_in_debug_output() {
    let raw_secret = constants_str::NEVER_PRINT_THIS_VALUE;
    let password = password(raw_secret);
    let jwt_secret =
        crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret::new(admin_secret(raw_secret));
    let access_token =
        crate::std_admin_access_token::StdAdminAccessToken::try_from(raw_secret.to_owned())
            .expect(constants_str::DIAGNOSTIC_E295277C);
    assert!(!format!("{password:?}").contains(raw_secret));
    assert!(!format!("{jwt_secret:?}").contains(raw_secret));
    assert!(!format!("{access_token:?}").contains(raw_secret));
}
#[test]
fn test_generated_token_hash_is_stable_and_does_not_expose_token() {
    let token = crate::admin_opaque_token::AdminOpaqueToken::new(admin_secret(
        constants_str::FIXED_TEST_TOKEN,
    ));
    let hash = crate::hash_opaque_token::hash_opaque_token(&token)
        .expect(constants_str::DIAGNOSTIC_3AF32394);
    assert_eq!(hash.expose().as_ref(), constants_str::VALUE_9CF3E4A3);
    assert!(!format!("{hash:?}").contains(constants_str::FIXED_TEST_TOKEN));
}
#[test]
fn test_cookie_policy_marks_only_secret_tokens_http_only() {
    let access = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Access,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ACCESS),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(60),
        crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(true),
    );
    let csrf = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::CSRF),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(60),
        crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(true),
    );
    assert!(
        access
            .as_ref()
            .is_ok_and(|cookie| cookie.as_ref().contains(constants_str::VALUE_A0820391))
    );
    assert!(
        access
            .as_ref()
            .is_ok_and(|cookie| cookie.as_ref().contains(constants_str::VALUE_1BCED1D0))
    );
    assert!(
        access
            .as_ref()
            .is_ok_and(|cookie| cookie.as_ref().contains(constants_str::VALUE_DD7C3F04))
    );
    assert!(
        csrf.as_ref()
            .is_ok_and(|cookie| !cookie.as_ref().contains(constants_str::VALUE_A0820391))
    );
    assert!(
        csrf.as_ref()
            .is_ok_and(|cookie| cookie.as_ref().contains(constants_str::VALUE_1BCED1D0))
    );
}
#[test]
fn test_cookie_builder_does_not_return_error_text_as_cookie() {
    let oversized_value = constants_str::ACCESS.repeat(8192usize);
    let cookie = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Access,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(oversized_value.as_str()),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(60),
        crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(true),
    );
    assert_eq!(
        cookie,
        Err(crate::admin_secret_text_error::AdminSecretTextError::TooLong)
    );
}
#[test]
fn test_cookie_parser_matches_complete_cookie_name() {
    let mut headers = http::HeaderMap::new();
    let _previous = headers.insert(
        http::header::COOKIE,
        http::HeaderValue::from_static(
            constants_str::OTHER_1_ADMIN_ACCESS_TOKEN_EXPECTED_ADMIN_ACCESS_TOKEN_SUFFIX_WRONG,
        ),
    );
    assert_eq!(
        crate::find_admin_cookie::find_admin_cookie(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
            crate::admin_cookie_kind::AdminCookieKind::Access,
        ),
        Some(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            constants_str::VALUE_CEA23DD4
        ))
    );
}
#[test]
fn test_administrator_login_format_accepts_only_database_compatible_values() {
    let valid = server_admin_contract::admin_login::AdminLogin::try_from(
        constants_str::ADMIN_USER_1.to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_078C759D);
    assert_eq!(valid.as_ref(), constants_str::ADMIN_USER_1);
    let _uppercase_error =
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::ADMIN.to_owned())
            .expect_err(constants_str::VALUE_5FA1C6E2);
    let _short_error =
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::AB.to_owned())
            .expect_err(constants_str::VALUE_B78D42A9);
}
#[test]
fn test_access_token_round_trip_checks_issuer_and_audience() {
    let claims = crate::admin_access_claims::AdminAccessClaims::new(
        server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(7)
            .expect(constants_str::DIAGNOSTIC_D6D3DA8A),
        crate::admin_session_id::AdminSessionId::try_from(1i64)
            .expect(constants_str::DIAGNOSTIC_05562DA0),
        crate::admin_unix_token_stream::AdminUnixTokenStream::from(1),
        crate::admin_unix_token_stream::AdminUnixTokenStream::from(4_102_444_800),
        config_lib::admin_token_issuer::AdminTokenIssuer::try_from(
            constants_str::TEST_ISSUER.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_FD6A65B0),
        config_lib::admin_token_audience::AdminTokenAudience::try_from(
            constants_str::TEST_AUDIENCE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_6E423E16),
    );
    let secret = crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret::new(admin_secret(
        constants_str::TEST_ONLY_SECRET_WITH_SUFFICIENT_ENTROPY,
    ));
    let serialized_claims =
        serde_json::to_value(&claims).expect(constants_str::DIAGNOSTIC_34CB1A36);
    [
        stringify!(audience),
        stringify!(expires_at),
        stringify!(issued_at),
        stringify!(issuer),
        stringify!(user_id),
        stringify!(session_id),
    ]
    .into_iter()
    .for_each(|field_name| assert!(serialized_claims.get(field_name).is_some()));
    [
        stringify!(aud),
        stringify!(exp),
        stringify!(iat),
        stringify!(iss),
        stringify!(sub),
        stringify!(jti),
    ]
    .into_iter()
    .for_each(|field_name| assert!(serialized_claims.get(field_name).is_none()));
    let token = crate::encode_access_token::encode_access_token(&claims, &secret)
        .expect(constants_str::DIAGNOSTIC_B41052BC);
    let issuer = config_lib::admin_token_issuer::AdminTokenIssuer::try_from(
        constants_str::TEST_ISSUER.to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_5EDC807F);
    let audience = config_lib::admin_token_audience::AdminTokenAudience::try_from(
        constants_str::TEST_AUDIENCE.to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_0C3975A1);
    let decoded =
        crate::decode_access_token::decode_access_token(&token, &secret, &issuer, &audience)
            .expect(constants_str::DIAGNOSTIC_0ED905FF);
    assert_eq!(
        decoded.user_id(),
        server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(7)
            .expect(constants_str::DIAGNOSTIC_5B88F22A)
    );
    assert_eq!(decoded.session_id(), claims.session_id());
    assert_eq!(decoded, claims);
    let rejected = |result, error_kind| {
        matches!(result, Err(crate::admin_access_token_error::AdminAccessTokenError::Token(source))
            if source.get_inner().kind() == &error_kind)
    };
    assert!(
        config_lib::admin_token_audience::AdminTokenAudience::try_from(
            constants_str::WRONG_AUDIENCE.to_owned()
        )
        .is_ok_and(|wrong_audience| rejected(
            crate::decode_access_token::decode_access_token(
                &token,
                &secret,
                &issuer,
                &wrong_audience
            ),
            jsonwebtoken::errors::ErrorKind::InvalidAudience
        ))
    );
    assert!(
        config_lib::admin_token_issuer::AdminTokenIssuer::try_from(
            constants_str::TEST_AUDIENCE.to_owned()
        )
        .is_ok_and(|wrong_issuer| rejected(
            crate::decode_access_token::decode_access_token(
                &token,
                &secret,
                &wrong_issuer,
                &audience
            ),
            jsonwebtoken::errors::ErrorKind::InvalidIssuer
        ))
    );
    let wrong_secret = crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret::new(admin_secret(
        constants_str::FIXED_TEST_TOKEN,
    ));
    assert!(rejected(
        crate::decode_access_token::decode_access_token(&token, &wrong_secret, &issuer, &audience),
        jsonwebtoken::errors::ErrorKind::InvalidSignature
    ));
    assert!(
        crate::std_admin_access_token::StdAdminAccessToken::try_from(constants_str::X.to_owned())
            .is_ok_and(|malformed_token| rejected(
                crate::decode_access_token::decode_access_token(
                    &malformed_token,
                    &secret,
                    &issuer,
                    &audience
                ),
                jsonwebtoken::errors::ErrorKind::InvalidToken
            ))
    );
    assert!(
        jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(
                secrecy::ExposeSecret::expose_secret(secret.get_inner().as_ref()).as_bytes()
            )
        )
        .is_ok_and(
            |encoded| crate::std_admin_access_token::StdAdminAccessToken::try_from(encoded)
                .is_ok_and(|disallowed_token| rejected(
                    crate::decode_access_token::decode_access_token(
                        &disallowed_token,
                        &secret,
                        &issuer,
                        &audience
                    ),
                    jsonwebtoken::errors::ErrorKind::InvalidAlgorithm
                ))
        )
    );
}

#[test]
fn test_admin_auth_ttl_values_preserve_nonzero_u64_boundaries() {
    [
        (
            0u64,
            Err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero),
        ),
        (1u64, Ok(1u64)),
        (2u64, Ok(2u64)),
        (u64::MAX, Ok(u64::MAX)),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        [
            crate::std_admin_auth_ttl_seconds::StdAdminAuthTtlSeconds::try_from(value)
                .map(std::num::NonZeroU64::from)
                .map(std::num::NonZeroU64::get),
            crate::std_admin_access_ttl_seconds::StdAdminAccessTtlSeconds::try_from(value).map(
                |std_admin_access_ttl_seconds| {
                    assert_eq!(std_admin_access_ttl_seconds.get(), value);
                    std_admin_access_ttl_seconds.get_inner().get()
                },
            ),
            crate::std_admin_refresh_ttl_seconds::StdAdminRefreshTtlSeconds::try_from(value).map(
                |std_admin_refresh_ttl_seconds| {
                    assert_eq!(std_admin_refresh_ttl_seconds.get(), value);
                    std_admin_refresh_ttl_seconds.get_inner().get()
                },
            ),
        ]
        .into_iter()
        .fold((), |(), result| assert_eq!(result, expected));
    });
}

#[test]
fn test_admin_session_limit_preserves_nonzero_usize_boundaries() {
    [
        (
            0usize,
            Err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero),
        ),
        (1usize, Ok(1usize)),
        (2usize, Ok(2usize)),
        (usize::MAX, Ok(usize::MAX)),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        assert_eq!(
            crate::std_admin_session_limit::StdAdminSessionLimit::try_from(value).map(
                |std_admin_session_limit| {
                    assert_eq!(std_admin_session_limit.get(), value);
                    std_admin_session_limit.get_inner().get()
                }
            ),
            expected
        );
    });
}

#[test]
fn test_admin_failure_threshold_preserves_signed_positive_boundaries() {
    [
        (
            i64::MIN,
            Err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero),
        ),
        (
            -1i64,
            Err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero),
        ),
        (
            0i64,
            Err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero),
        ),
        (1i64, Ok(1i64)),
        (2i64, Ok(2i64)),
        (i64::MAX, Ok(i64::MAX)),
    ]
    .into_iter()
    .fold((), |(), (value, expected)| {
        assert_eq!(
            crate::std_admin_failure_threshold::StdAdminFailureThreshold::try_from(value).map(
                |std_admin_failure_threshold| {
                    assert_eq!(std_admin_failure_threshold.get(), value);
                    std_admin_failure_threshold.get_inner().get()
                }
            ),
            expected
        );
    });
}

#[test]
fn test_admin_recent_login_failures_reach_exact_thresholds_without_overflow() {
    [
        (1i64, [false, false, false, true, true, true, true]),
        (2i64, [false, false, false, false, true, true, true]),
        (i64::MAX, [false, false, false, false, false, false, true]),
    ].into_iter().fold((), |(), (threshold_value, expected)| {
        let std_admin_failure_threshold_result = crate::std_admin_failure_threshold::StdAdminFailureThreshold::try_from(threshold_value);
        assert!(std_admin_failure_threshold_result.as_ref().err().is_none());
        if let Ok(std_admin_failure_threshold) = std_admin_failure_threshold_result {
            [i64::MIN, -1i64, 0i64, 1i64, 2i64, i64::MAX - 1i64, i64::MAX]
                .into_iter().zip(expected).fold((), |(), (count, expected)| {
                    assert_eq!(crate::admin_recent_login_failure_count::AdminRecentLoginFailureCount::from(count)
                        .reached(std_admin_failure_threshold).get(), expected);
                });
        }
    });
}

#[test]
fn test_admin_shared_semaphore_preserves_capacity_and_shared_permit_lifecycle() {
    [1usize, 2usize, 7usize].into_iter().fold((), |(), capacity| {
        let non_zero_usize = std::num::NonZeroUsize::new(capacity);
        let capacity_u32_result = u32::try_from(capacity);
        assert!(non_zero_usize.is_some());
        assert!(capacity_u32_result.as_ref().err().is_none());
        if let (Some(non_zero_usize), Ok(capacity_u32)) = (non_zero_usize, capacity_u32_result) {
            let original = crate::admin_shared_semaphore_arc::AdminSharedSemaphoreArc::new(
                crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(non_zero_usize),
            );
            let cloned = original.clone();
            assert!(std::sync::Arc::ptr_eq(original.get_inner(), cloned.get_inner()));
            assert_eq!(original.get_inner().available_permits(), capacity);
            assert!(cloned.get_inner().try_acquire_many(capacity_u32).is_ok_and(|permit| {
                assert_eq!(original.get_inner().available_permits(), 0usize);
                assert!(matches!(original.get_inner().try_acquire(), Err(tokio::sync::TryAcquireError::NoPermits)));
                drop(permit);
                original.get_inner().available_permits() == capacity
            }));
            original.get_inner().close();
            assert!(cloned.get_inner().is_closed());
            assert!(matches!(cloned.get_inner().try_acquire(), Err(tokio::sync::TryAcquireError::Closed)));
        }
    });
}

#[test]
fn test_admin_access_token_preserves_byte_bounds_content_and_exact_redaction() {
    [
        (String::new(), 0usize, 0usize, 'x'),
        (constants_str::X.to_owned(), 1usize, 1usize, 'x'),
        (
            constants_str::X.repeat(8_191usize),
            8_191usize,
            8_191usize,
            'x',
        ),
        (
            constants_str::X.repeat(8_192usize),
            8_192usize,
            8_192usize,
            'x',
        ),
        (
            '\u{00e9}'.to_string().repeat(4_096usize),
            8_192usize,
            4_096usize,
            '\u{00e9}',
        ),
        (
            char::MAX.to_string().repeat(2_048usize),
            8_192usize,
            2_048usize,
            char::MAX,
        ),
    ]
    .into_iter()
    .fold(
        (),
        |(), (text, expected_bytes, expected_characters, expected_character)| {
            assert!(
                crate::std_admin_access_token::StdAdminAccessToken::try_from(text).is_ok_and(
                    |access_token| {
                        access_token.as_ref().as_str().len() == expected_bytes
                            && access_token.as_ref().as_str().chars().count() == expected_characters
                            && access_token
                                .as_ref()
                                .as_str()
                                .chars()
                                .all(|character| character == expected_character)
                            && format!("{access_token:?}") == constants_str::REDACTED_ALT_3
                    }
                )
            );
        },
    );
    [
        constants_str::X.repeat(8_193usize),
        [char::MAX.to_string().repeat(2_048usize).as_str(), constants_str::X].concat(),
    ].into_iter().fold((), |(), text| {
        assert!(crate::std_admin_access_token::StdAdminAccessToken::try_from(text).is_err_and(|error| {
            matches!(error, crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooLong { len: 8_193usize, max: 8_192usize })
        }));
    });
    assert!(
        crate::std_admin_access_token::StdAdminAccessToken::try_from(
            constants_str::TEST_TEXT_WITH_NUL.to_owned()
        )
        .is_ok_and(|access_token| {
            access_token.as_ref().as_str() == constants_str::TEST_TEXT_WITH_NUL
                && format!("{access_token:?}") == constants_str::REDACTED_ALT_3
        })
    );
}

#[test]
fn test_cleared_admin_cookies_empty_values_and_expire_every_kind_with_security_attributes() {
    assert!(
        [
            (
                crate::admin_cookie_kind::AdminCookieKind::Access,
                constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
                true
            ),
            (
                crate::admin_cookie_kind::AdminCookieKind::Refresh,
                constants_str::ADMIN_REFRESH_TOKEN,
                true
            ),
            (
                crate::admin_cookie_kind::AdminCookieKind::Csrf,
                constants_str::ADMIN_CSRF_TOKEN,
                false
            ),
        ]
        .into_iter()
        .all(|(admin_cookie_kind, expected_name, http_only)| {
            [false, true].into_iter().all(|secure| {
                crate::clear_admin_cookie::clear_admin_cookie(
                    admin_cookie_kind,
                    crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(secure),
                )
                .is_ok_and(|std_admin_cookie| {
                    let mut parts = std_admin_cookie.as_ref().split(';').map(str::trim);
                    let value_matches = parts.next().is_some_and(|part| {
                        part.split_once('=')
                            .is_some_and(|(name, value)| name == expected_name && value.is_empty())
                    });
                    let attributes = parts.collect::<Vec<_>>();
                    value_matches
                        && attributes.len() == 3usize + usize::from(http_only) + usize::from(secure)
                        && attributes.iter().any(|attribute| {
                            attribute.split_once('=').is_some_and(|(name, value)| {
                                name.eq_ignore_ascii_case(constants_str::PATH_ALT_5)
                                    && value == constants_str::SLASH
                            })
                        })
                        && constants_str::MAX_AGE_31536000_INCLUDESUBDOMAINS
                            .split_once('=')
                            .is_some_and(|(age_name, _)| {
                                attributes.iter().any(|attribute| {
                                    attribute.split_once('=').is_some_and(|(name, value)| {
                                        name.eq_ignore_ascii_case(age_name)
                                            && value.chars().eq(std::iter::once('0'))
                                    })
                                })
                            })
                        && attributes.contains(&constants_str::VALUE_DD7C3F04)
                        && attributes.contains(&constants_str::VALUE_A0820391) == http_only
                        && attributes.contains(&constants_str::VALUE_1BCED1D0) == secure
                })
            })
        })
    );
}

#[test]
fn test_admin_sign_in_row_conversion_preserves_fields_and_secret_redaction() {
    assert!([1i64, i64::MAX].into_iter().all(|identifier| {
        [false, true].into_iter().all(|banned| {
            [constants_str::EMPTY, constants_str::FIXED_TEST_TOKEN]
                .into_iter()
                .all(|text| {
                    crate::admin_sign_in_user::AdminSignInUser::try_from((
                        identifier,
                        text.to_owned(),
                        banned,
                    ))
                    .is_ok_and(|admin_sign_in_user| {
                        let redacted = !format!("{admin_sign_in_user:?}")
                            .contains(constants_str::FIXED_TEST_TOKEN);
                        let (admin_user_record_id, admin_password_hash, std_admin_bool) =
                            <(
                                server_admin_core::admin_user_record_id::AdminUserRecordId,
                                crate::admin_password_hash::AdminPasswordHash,
                                server_admin_core::std_admin_bool::StdAdminBool,
                            )>::from(admin_sign_in_user);
                        redacted
                            && admin_user_record_id.get() == identifier
                            && admin_password_hash.expose().as_ref() == text
                            && std_admin_bool.get() == banned
                    })
                })
        })
    }));
}

#[test]
fn test_admin_sign_in_row_conversion_preserves_validation_sources_and_error_priority() {
    assert!([0i64, -1i64, i64::MIN].into_iter().all(|identifier| {
        crate::admin_sign_in_user::AdminSignInUser::try_from((identifier, String::new(), false))
            .is_err_and(|error| matches!(error.into_inner(), sqlx::Error::Decode(source)
                if source.downcast_ref::<server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error>()
                    == Some(&server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid)))
    }));
    let maximum = constants_usize::VALUE_1_048_576;
    assert!([0i64, 1i64].into_iter().all(|identifier| {
        crate::admin_sign_in_user::AdminSignInUser::try_from((identifier, constants_str::X.repeat(maximum + 1usize), false))
            .is_err_and(|error| {
                let sqlx::Error::Decode(source) = error.into_inner() else { return false; };
                if identifier == 0i64 {
                    source.downcast_ref::<server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error>()
                        == Some(&server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid)
                } else {
                    source.downcast_ref::<bounded_types::bounded_string_error::BoundedStringError>().is_some_and(|bounded_string_error| {
                        matches!(bounded_string_error, bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { actual_length, maximum_length }
                            if *actual_length == bounded_types::bounded_len::BoundedLen::from(maximum + 1usize)
                                && *maximum_length == bounded_types::bounded_len::BoundedLen::from(maximum))
                    })
                }
            })
    }));
}

#[test]
fn test_settings_form_deserialization_preserves_fields_and_rejects_invalid_shapes() {
    let payload = || {
        serde_json::json!({
            (stringify!(default_admin_route)): server_admin_contract::admin_page::AdminPage::Profile.path().as_ref(),
            (stringify!(main_logo)): constants_str::LOGIN,
            (stringify!(organization_contacts)): constants_str::DISPLAY_NAME,
            (stringify!(organization_name)): constants_str::HELLOWORLD,
            (stringify!(primary_color)): constants_str::X,
            (stringify!(site_name)): constants_str::ERROR,
            (stringify!(support_url)): constants_str::HTTPS_EXAMPLE_COM,
            (stringify!(tab_title)): constants_str::JSON,
        })
    };
    assert!(
        serde_json::from_value::<crate::settings_form::SettingsForm>(payload()).is_ok_and(
            |settings_form| {
                let (
                    default_admin_route,
                    main_logo,
                    organization_contacts,
                    organization_name,
                    primary_color,
                    site_name,
                    support_url,
                    tab_title,
                ) = settings_form.into_parts();
                default_admin_route.as_ref()
                    == server_admin_contract::admin_page::AdminPage::Profile
                        .path()
                        .as_ref()
                    && main_logo.as_str() == constants_str::LOGIN
                    && organization_contacts.as_str() == constants_str::DISPLAY_NAME
                    && organization_name.as_str() == constants_str::HELLOWORLD
                    && primary_color.as_str() == constants_str::X
                    && site_name.as_ref() == constants_str::ERROR
                    && support_url.as_str() == constants_str::HTTPS_EXAMPLE_COM
                    && tab_title.as_str() == constants_str::JSON
            }
        )
    );
    assert!(
        [
            stringify!(default_admin_route),
            stringify!(main_logo),
            stringify!(organization_contacts),
            stringify!(organization_name),
            stringify!(primary_color),
            stringify!(site_name),
            stringify!(support_url),
            stringify!(tab_title),
        ]
        .into_iter()
        .all(|field| {
            [false, true].into_iter().all(|missing| {
                let mut body = payload();
                let Some(object) = body.as_object_mut() else {
                    return false;
                };
                if missing {
                    let _removed = object.remove(field);
                } else {
                    let _previous = object.insert(field.to_owned(), serde_json::Value::Null);
                }
                serde_json::from_value::<crate::settings_form::SettingsForm>(body)
                    .is_err_and(|error| error.is_data())
            })
        })
    );
    let mut body = payload();
    let inserted = body.as_object_mut().is_some_and(|object| {
        object
            .insert(
                stringify!(unexpected).to_owned(),
                serde_json::Value::Bool(true),
            )
            .is_none()
    });
    assert!(inserted);
    assert!(
        serde_json::from_value::<crate::settings_form::SettingsForm>(body)
            .is_err_and(|error| error.is_data())
    );
}

#[test]
fn test_refresh_token_exposure_preserves_secret_and_debug_remains_redacted() {
    assert!(
        [
            constants_str::FIXED_TEST_TOKEN,
            constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES,
        ]
        .into_iter()
        .all(|text| {
            let admin_refresh_token = crate::admin_refresh_token::AdminRefreshToken::new(
                crate::admin_opaque_token::AdminOpaqueToken::new(admin_secret(text)),
            );
            let debug = format!("{admin_refresh_token:?}");
            admin_refresh_token.expose().as_ref() == text
                && debug.contains(constants_str::REDACTED_ALT_3)
                && !debug.contains(text)
        })
    );
}

fn test_admin_form_rejects_invalid_fields<Form, const FIELD_COUNT: usize>(
    serde_json_value: &serde_json::Value,
    fields: [&str; FIELD_COUNT],
) where
    Form: serde::de::DeserializeOwned,
{
    assert!(fields.into_iter().all(|field| {
        let wrong_type = if serde_json_value
            .get(field)
            .is_some_and(serde_json::Value::is_boolean)
        {
            serde_json::json!(1i64)
        } else {
            serde_json::Value::Bool(true)
        };
        [
            None,
            Some(serde_json::Value::Null),
            Some(wrong_type),
            Some(serde_json::Value::String(String::new())),
        ]
        .into_iter()
        .all(|replacement| {
            let mut body = serde_json_value.clone();
            let Some(object) = body.as_object_mut() else {
                return false;
            };
            if let Some(value) = replacement {
                let _previous = object.insert(field.to_owned(), value);
            } else {
                let _removed = object.remove(field);
            }
            serde_json::from_value::<Form>(body).is_err_and(|error| error.is_data())
        })
    }));
    let mut body = serde_json_value.clone();
    assert!(body.as_object_mut().is_some_and(|object| {
        object
            .insert(
                stringify!(unexpected).to_owned(),
                serde_json::Value::Bool(true),
            )
            .is_none()
    }));
    assert!(serde_json::from_value::<Form>(body).is_err_and(|error| error.is_data()));
}

#[test]
fn test_create_role_form_preserves_name_and_rejects_invalid_fields() {
    let body = serde_json::json!({ (stringify!(name)): constants_str::ROOT });
    assert!(
        serde_json::from_value::<crate::create_role_form::CreateRoleForm>(body.clone())
            .is_ok_and(|form| form.get_name().as_ref() == constants_str::ROOT)
    );
    test_admin_form_rejects_invalid_fields::<crate::create_role_form::CreateRoleForm, 1usize>(
        &body,
        [stringify!(name)],
    );
}

#[test]
fn test_create_user_form_preserves_fields_and_rejects_invalid_fields() {
    assert!(server_admin_contract::admin_new_password::AdminNewPassword::try_from(
        server_admin_contract::admin_password_entropy::AdminPasswordEntropy::from([0u8; 32usize])
    ).is_ok_and(|password| {
        let body = serde_json::json!({
            (stringify!(display_name)): constants_str::ADMIN,
            (stringify!(login)): constants_str::ROOT,
            (stringify!(password)): password.as_ref(),
        });
        let preserved = serde_json::from_value::<crate::create_user_form::CreateUserForm>(body.clone())
            .is_ok_and(|form| form.get_display_name().as_ref() == constants_str::ADMIN
                && form.get_login().as_ref() == constants_str::ROOT
                && form.get_password().as_ref() == password.as_ref());
        test_admin_form_rejects_invalid_fields::<crate::create_user_form::CreateUserForm, 3usize>(
            &body, [stringify!(display_name), stringify!(login), stringify!(password)]);
        preserved
    }));
}

#[test]
fn test_change_password_form_preserves_fields_and_rejects_invalid_fields() {
    assert!(
        server_admin_contract::admin_new_password::AdminNewPassword::try_from(
            server_admin_contract::admin_password_entropy::AdminPasswordEntropy::from(
                [0u8; 32usize]
            )
        )
        .is_ok_and(|password| {
            let body = serde_json::json!({
                (stringify!(current_password)): constants_str::X,
                (stringify!(new_password)): password.as_ref(),
            });
            let preserved =
                serde_json::from_value::<crate::change_password_form::ChangePasswordForm>(
                    body.clone(),
                )
                .is_ok_and(|form| {
                    form.get_current_password().as_ref() == constants_str::X
                        && form.get_new_password().as_ref() == password.as_ref()
                });
            test_admin_form_rejects_invalid_fields::<
                crate::change_password_form::ChangePasswordForm,
                2usize,
            >(
                &body,
                [stringify!(current_password), stringify!(new_password)],
            );
            preserved
        })
    );
}

#[test]
fn test_sign_in_form_preserves_fields_and_rejects_invalid_fields() {
    let body = serde_json::json!({
        (stringify!(login)): constants_str::ROOT,
        (stringify!(password)): constants_str::X,
    });
    assert!(
        serde_json::from_value::<crate::sign_in_form::SignInForm>(body.clone())
            .is_ok_and(|form| form.get_login().as_ref() == constants_str::ROOT
                && form.get_password().as_ref() == constants_str::X)
    );
    test_admin_form_rejects_invalid_fields::<crate::sign_in_form::SignInForm, 2usize>(
        &body,
        [stringify!(login), stringify!(password)],
    );
}

#[test]
fn test_role_id_form_preserves_both_confirmation_values_and_rejects_invalid_fields() {
    [false, true].into_iter().fold((), |(), confirmation| {
        [1i64, i64::MAX].into_iter().fold((), |(), role_id| {
            let body = serde_json::json!({ (stringify!(role_id)): role_id, (stringify!(confirmation)): confirmation });
            assert!(serde_json::from_value::<crate::role_id_form::RoleIdForm>(body.clone())
                .is_ok_and(|form| i64::from(*form.get_role_id()) == role_id
                    && bool::from(*form.get_confirmation()) == confirmation));
            test_admin_form_rejects_invalid_fields::<crate::role_id_form::RoleIdForm, 2usize>(
                &body, [stringify!(role_id), stringify!(confirmation)]);
        });
        assert!([i64::MIN, -1i64, 0i64].into_iter().all(|role_id| {
            serde_json::from_value::<crate::role_id_form::RoleIdForm>(serde_json::json!({
                (stringify!(role_id)): role_id, (stringify!(confirmation)): confirmation,
            })).is_err_and(|error| error.is_data())
        }));
    });
}

#[test]
fn test_revoke_session_form_preserves_positive_integer_and_both_confirmation_values() {
    [false, true].into_iter().fold((), |(), confirmation| {
        [1i64, i64::MAX]
            .into_iter().fold((), |(), session_id| {
                let body = serde_json::json!({ (stringify!(session_id)): session_id, (stringify!(confirmation)): confirmation });
                assert!(serde_json::from_value::<crate::revoke_session_form::RevokeSessionForm>(body.clone())
                    .is_ok_and(|form| i64::from(*form.get_session_id()) == session_id
                        && bool::from(*form.get_confirmation()) == confirmation));
                test_admin_form_rejects_invalid_fields::<crate::revoke_session_form::RevokeSessionForm, 1usize>(
                    &body, [stringify!(confirmation)]);
            });
        assert!([
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::Value::Bool(true)),
            Some(serde_json::Value::String(constants_str::X.repeat(65usize))),
        ].into_iter().all(|session_id| {
            let mut body = serde_json::json!({ (stringify!(confirmation)): confirmation });
            if let Some(value) = session_id {
                let Some(object) = body.as_object_mut() else { return false; };
                let _previous = object.insert(stringify!(session_id).to_owned(), value);
            }
            serde_json::from_value::<crate::revoke_session_form::RevokeSessionForm>(body)
                .is_err_and(|error| error.is_data())
        }));
    });
}

#[test]
fn test_user_id_form_preserves_both_confirmation_values_and_rejects_invalid_fields() {
    [false, true].into_iter().fold((), |(), confirmation| {
        [1i64, i64::MAX].into_iter().fold((), |(), user_id| {
            let body = serde_json::json!({ (stringify!(user_id)): user_id, (stringify!(confirmation)): confirmation });
            assert!(serde_json::from_value::<crate::user_id_form::UserIdForm>(body.clone())
                .is_ok_and(|form| i64::from(*form.get_user_id()) == user_id
                    && bool::from(*form.get_confirmation()) == confirmation));
            test_admin_form_rejects_invalid_fields::<crate::user_id_form::UserIdForm, 2usize>(
                &body, [stringify!(user_id), stringify!(confirmation)]);
        });
        assert!([i64::MIN, -1i64, 0i64].into_iter().all(|user_id| {
            serde_json::from_value::<crate::user_id_form::UserIdForm>(serde_json::json!({
                (stringify!(user_id)): user_id, (stringify!(confirmation)): confirmation,
            })).is_err_and(|error| error.is_data())
        }));
    });
}

#[test]
fn test_user_password_form_preserves_fields_and_rejects_invalid_fields() {
    assert!(
        server_admin_contract::admin_new_password::AdminNewPassword::try_from(
            server_admin_contract::admin_password_entropy::AdminPasswordEntropy::from(
                [0u8; 32usize]
            )
        )
        .is_ok_and(|password| {
            [1i64, i64::MAX].into_iter().fold((), |(), user_id| {
                let body = serde_json::json!({
                    (stringify!(password)): password.as_ref(), (stringify!(user_id)): user_id,
                });
                assert!(
                    serde_json::from_value::<crate::user_password_form::UserPasswordForm>(
                        body.clone()
                    )
                    .is_ok_and(|form| form.get_password().as_ref() == password.as_ref()
                        && i64::from(*form.get_user_id()) == user_id)
                );
                test_admin_form_rejects_invalid_fields::<
                    crate::user_password_form::UserPasswordForm,
                    2usize,
                >(&body, [stringify!(password), stringify!(user_id)]);
            });
            [i64::MIN, -1i64, 0i64].into_iter().all(|user_id| {
                serde_json::from_value::<crate::user_password_form::UserPasswordForm>(
                    serde_json::json!({
                        (stringify!(password)): password.as_ref(), (stringify!(user_id)): user_id,
                    }),
                )
                .is_err_and(|error| error.is_data())
            })
        })
    );
}

#[test]
fn test_update_role_form_preserves_fields_and_rejects_invalid_fields() {
    [1i64, i64::MAX].into_iter().fold((), |(), role_id| {
        let body = serde_json::json!({ (stringify!(name)): constants_str::ROOT, (stringify!(role_id)): role_id });
        assert!(serde_json::from_value::<crate::update_role_form::UpdateRoleForm>(body.clone())
            .is_ok_and(|form| form.get_name().as_ref() == constants_str::ROOT
                && i64::from(*form.get_role_id()) == role_id));
        test_admin_form_rejects_invalid_fields::<crate::update_role_form::UpdateRoleForm, 2usize>(
            &body, [stringify!(name), stringify!(role_id)]);
    });
    assert!([i64::MIN, -1i64, 0i64].into_iter().all(|role_id| {
        serde_json::from_value::<crate::update_role_form::UpdateRoleForm>(serde_json::json!({
            (stringify!(name)): constants_str::ROOT, (stringify!(role_id)): role_id,
        }))
        .is_err_and(|error| error.is_data())
    }));
}

#[test]
fn test_update_user_form_preserves_fields_and_rejects_invalid_fields() {
    [1i64, i64::MAX].into_iter().fold((), |(), user_id| {
        let body = serde_json::json!({
            (stringify!(display_name)): constants_str::ADMIN,
            (stringify!(login)): constants_str::ROOT, (stringify!(user_id)): user_id,
        });
        assert!(
            serde_json::from_value::<crate::update_user_form::UpdateUserForm>(body.clone())
                .is_ok_and(
                    |form| form.get_display_name().as_ref() == constants_str::ADMIN
                        && form.get_login().as_ref() == constants_str::ROOT
                        && i64::from(*form.get_user_id()) == user_id
                )
        );
        test_admin_form_rejects_invalid_fields::<crate::update_user_form::UpdateUserForm, 3usize>(
            &body,
            [
                stringify!(display_name),
                stringify!(login),
                stringify!(user_id),
            ],
        );
    });
    assert!([i64::MIN, -1i64, 0i64].into_iter().all(|user_id| {
        serde_json::from_value::<crate::update_user_form::UpdateUserForm>(serde_json::json!({
            (stringify!(display_name)): constants_str::ADMIN,
            (stringify!(login)): constants_str::ROOT, (stringify!(user_id)): user_id,
        }))
        .is_err_and(|error| error.is_data())
    }));
}

#[test]
fn test_user_ban_form_preserves_both_ban_values_and_rejects_invalid_fields() {
    [false, true].into_iter().fold((), |(), is_banned| {
        [1i64, i64::MAX].into_iter().fold((), |(), user_id| {
            let body = serde_json::json!({ (stringify!(user_id)): user_id, (stringify!(is_banned)): is_banned });
            assert!(serde_json::from_value::<crate::user_ban_form::UserBanForm>(body.clone())
                .is_ok_and(|form| i64::from(*form.get_user_id()) == user_id
                    && bool::from(*form.get_is_banned()) == is_banned));
            test_admin_form_rejects_invalid_fields::<crate::user_ban_form::UserBanForm, 2usize>(
                &body, [stringify!(user_id), stringify!(is_banned)]);
        });
        assert!([i64::MIN, -1i64, 0i64].into_iter().all(|user_id| {
            serde_json::from_value::<crate::user_ban_form::UserBanForm>(serde_json::json!({
                (stringify!(user_id)): user_id, (stringify!(is_banned)): is_banned,
            })).is_err_and(|error| error.is_data())
        }));
    });
}

#[test]
fn test_role_rules_form_preserves_flattened_fields_and_rejects_invalid_values() {
    [constants_str::EMPTY.to_owned(), constants_str::ROOT.to_owned(), constants_str::X.repeat(8_192usize)]
        .into_iter().fold((), |(), expected_rule_ids| {
            [1i64, i64::MAX].into_iter().fold((), |(), role_id| {
                let body = serde_json::json!({
                    (stringify!(role_id)): role_id, (stringify!(expected_rule_ids)): expected_rule_ids,
                    (constants_str::ADMIN): constants_str::X, (constants_str::ROOT): constants_str::EMPTY,
                });
                assert!(serde_json::from_value::<crate::role_rules_form::RoleRulesForm>(body)
                    .is_ok_and(|form| i64::from(*form.get_role_id()) == role_id
                        && form.get_expected_rule_ids().as_str() == expected_rule_ids
                        && form.get_selected().len().get() == 2usize
                        && form.get_selected().iter().all(|(key, text)| {
                            match key.get_inner().as_str() {
                                constants_str::ADMIN => text.as_str() == constants_str::X,
                                constants_str::ROOT => text.as_str() == constants_str::EMPTY,
                                _ => false,
                            }
                        })));
            });
        });
    let valid = serde_json::json!({ (stringify!(role_id)): 1i64, (stringify!(expected_rule_ids)): constants_str::EMPTY });
    assert!(
        serde_json::from_value::<crate::role_rules_form::RoleRulesForm>(valid)
            .is_ok_and(|form| form.get_selected().len().get() == 0usize)
    );
    assert!([
        serde_json::json!({ (stringify!(expected_rule_ids)): constants_str::EMPTY }),
        serde_json::json!({ (stringify!(role_id)): 1i64 }),
        serde_json::json!({ (stringify!(role_id)): 0i64, (stringify!(expected_rule_ids)): constants_str::EMPTY }),
        serde_json::json!({ (stringify!(role_id)): 1i64, (stringify!(expected_rule_ids)): null }),
        serde_json::json!({ (stringify!(role_id)): 1i64, (stringify!(expected_rule_ids)): true }),
        serde_json::json!({ (stringify!(role_id)): 1i64, (stringify!(expected_rule_ids)): constants_str::X.repeat(8_193usize) }),
        serde_json::json!({ (stringify!(role_id)): 1i64, (stringify!(expected_rule_ids)): constants_str::EMPTY, (constants_str::ADMIN): null }),
        serde_json::json!({ (stringify!(role_id)): 1i64, (stringify!(expected_rule_ids)): constants_str::EMPTY, (constants_str::ADMIN): true }),
    ].into_iter().all(|body| serde_json::from_value::<crate::role_rules_form::RoleRulesForm>(body)
        .is_err_and(|error| error.is_data())));
}

#[test]
fn test_admin_audit_action_serde_and_case_insensitive_parse_contract() {
    [
        (
            crate::admin_audit_action::AdminAuditAction::Create,
            constants_str::PG_CRUD_CREATE_RULE_ACTION,
        ),
        (
            crate::admin_audit_action::AdminAuditAction::Delete,
            constants_str::PG_CRUD_DELETE_RULE_ACTION,
        ),
        (
            crate::admin_audit_action::AdminAuditAction::Refresh,
            constants_str::REFRESH,
        ),
        (
            crate::admin_audit_action::AdminAuditAction::SignIn,
            constants_str::SIGN_IN,
        ),
        (
            crate::admin_audit_action::AdminAuditAction::SignOut,
            constants_str::SIGN_OUT,
        ),
        (
            crate::admin_audit_action::AdminAuditAction::Update,
            constants_str::PG_CRUD_UPDATE_RULE_ACTION,
        ),
    ]
    .into_iter()
    .fold((), |(), (variant, name)| {
        assert_eq!(
            serde_json::to_value(variant).ok(),
            Some(serde_json::Value::String(name.to_owned()))
        );
        assert_eq!(
            serde_json::from_value::<crate::admin_audit_action::AdminAuditAction>(
                serde_json::Value::String(name.to_owned())
            )
            .ok(),
            Some(variant)
        );
        assert!(
            [name.to_owned(), name.to_ascii_uppercase()]
                .into_iter()
                .all(|text| text
                    .parse::<crate::admin_audit_action::AdminAuditAction>()
                    .is_ok_and(|parsed| parsed == variant))
        );
        assert!(
            serde_json::from_value::<crate::admin_audit_action::AdminAuditAction>(
                serde_json::Value::String(name.to_ascii_uppercase())
            )
            .is_err_and(|error| error.is_data())
        );
    });
    assert!(
        [constants_str::EMPTY, constants_str::X, constants_str::SPACE]
            .into_iter()
            .all(|text| text
                .parse::<crate::admin_audit_action::AdminAuditAction>()
                .is_err())
    );
    assert!(
        [
            serde_json::Value::Null,
            serde_json::Value::Bool(true),
            serde_json::Value::String(constants_str::X.to_owned())
        ]
        .into_iter()
        .all(
            |wire| serde_json::from_value::<crate::admin_audit_action::AdminAuditAction>(wire)
                .is_err_and(|error| error.is_data())
        )
    );
}

#[test]
fn test_admin_audit_resource_serde_and_case_insensitive_parse_contract() {
    [
        (
            crate::admin_audit_resource::AdminAuditResource::AuditLog,
            constants_str::AUDIT_LOG_ALT,
        ),
        (
            crate::admin_audit_resource::AdminAuditResource::Rule,
            constants_str::RULE,
        ),
        (
            crate::admin_audit_resource::AdminAuditResource::Role,
            constants_str::ROLE,
        ),
        (
            crate::admin_audit_resource::AdminAuditResource::Session,
            constants_str::SESSION,
        ),
        (
            crate::admin_audit_resource::AdminAuditResource::SystemSettings,
            constants_str::SYSTEM_SETTINGS,
        ),
        (
            crate::admin_audit_resource::AdminAuditResource::User,
            constants_str::USER,
        ),
    ]
    .into_iter()
    .fold((), |(), (variant, name)| {
        assert_eq!(
            serde_json::to_value(variant).ok(),
            Some(serde_json::Value::String(name.to_owned()))
        );
        assert_eq!(
            serde_json::from_value::<crate::admin_audit_resource::AdminAuditResource>(
                serde_json::Value::String(name.to_owned())
            )
            .ok(),
            Some(variant)
        );
        assert!(
            [name.to_owned(), name.to_ascii_uppercase()]
                .into_iter()
                .all(|text| text
                    .parse::<crate::admin_audit_resource::AdminAuditResource>()
                    .is_ok_and(|parsed| parsed == variant))
        );
        assert!(
            serde_json::from_value::<crate::admin_audit_resource::AdminAuditResource>(
                serde_json::Value::String(name.to_ascii_uppercase())
            )
            .is_err_and(|error| error.is_data())
        );
    });
    assert!(
        [constants_str::EMPTY, constants_str::X, constants_str::SPACE]
            .into_iter()
            .all(|text| text
                .parse::<crate::admin_audit_resource::AdminAuditResource>()
                .is_err())
    );
    assert!(
        [
            serde_json::Value::Null,
            serde_json::Value::Bool(true),
            serde_json::Value::String(constants_str::X.to_owned())
        ]
        .into_iter()
        .all(|wire| serde_json::from_value::<
            crate::admin_audit_resource::AdminAuditResource,
        >(wire)
        .is_err_and(|error| error.is_data()))
    );
}

#[test]
fn test_runtime_authenticated_admin_preserves_distinct_fields_and_copy_accessors() {
    assert!([7i64, i64::MAX].into_iter().all(|identifier| {
        [false, true].into_iter().all(|required| {
            let fields = (
                server_admin_contract::admin_display_name::AdminDisplayName::try_from(constants_str::ADMIN.to_owned()),
                server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(identifier),
                server_admin_contract::admin_login::AdminLogin::try_from(constants_str::LOGIN.to_owned()),
                crate::admin_auth_rules::AdminAuthRules::try_from(vec![server_admin_contract::admin_rule::AdminRule::UsersRead, server_admin_contract::admin_rule::AdminRule::MetricsRead]),
                server_admin_contract::admin_role_name::AdminRoleName::try_from(constants_str::ROOT.to_owned()),
                server_admin_contract::admin_role_name::AdminRoleName::try_from(constants_str::USER.to_owned()),
            );
            let (Ok(display_name), Ok(id), Ok(login), Ok(rules), Ok(first_role), Ok(second_role)) = fields else {
                return false;
            };
            crate::runtime_admin_role_names::RuntimeAdminRoleNames::try_from(vec![first_role, second_role]).is_ok_and(|roles| {
                let session_id = crate::admin_session_id::AdminSessionId::from(id.value());
                let password_change_required = crate::admin_password_change_required::AdminPasswordChangeRequired::from(required);
                let administrator = crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin::new(display_name, id, login, rules, roles, session_id, password_change_required);
                let expected = serde_json::json!({
                    (stringify!(display_name)): constants_str::ADMIN,
                    (stringify!(id)): identifier,
                    (stringify!(login)): constants_str::LOGIN,
                    (stringify!(rules)): [server_admin_contract::admin_rule::AdminRule::UsersRead, server_admin_contract::admin_rule::AdminRule::MetricsRead],
                    (stringify!(roles)): [constants_str::ROOT, constants_str::USER],
                    (stringify!(session_id)): session_id,
                    (stringify!(password_change_required)): required,
                });
                let getters = serde_json::json!({
                    (stringify!(display_name)): administrator.get_display_name(),
                    (stringify!(id)): administrator.id(),
                    (stringify!(login)): administrator.get_login(),
                    (stringify!(rules)): administrator.get_rules(),
                    (stringify!(roles)): administrator.get_roles(),
                    (stringify!(session_id)): administrator.get_session_id(),
                    (stringify!(password_change_required)): administrator.password_change_required(),
                });
                getters == expected
                    && *administrator.get_id() == id
                    && *administrator.get_password_change_required() == password_change_required
                    && administrator.id() == id
                    && administrator.password_change_required() == password_change_required
                    && serde_json::to_value(administrator).is_ok_and(|wire| wire == expected)
            })
        })
    }));
}

#[test]
fn test_optional_settings_preserve_nonblank_values_clear_whitespace_and_reject_invalid_values() {
    fn optional_setting_matches<Value>(
        std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
        std_admin_bool: server_admin_core::std_admin_bool::StdAdminBool,
    ) -> server_admin_core::std_admin_bool::StdAdminBool
    where
        Value: TryFrom<String> + AsRef<str>,
    {
        server_admin_core::std_admin_bool::StdAdminBool::from(
            crate::admin_html_form_text::AdminHtmlFormText::try_from(
                std_admin_str_ref.get().to_owned(),
            )
            .is_ok_and(|admin_html_form_text| {
                match (
                    crate::optional_setting_impl::optional_setting_impl::<Value, _>(
                        admin_html_form_text,
                    ),
                    std_admin_bool.get(),
                ) {
                    (Ok(None), false) => true,
                    (Ok(Some(value)), true) => value.as_ref() == std_admin_str_ref.get(),
                    _ => false,
                }
            }),
        )
    }
    let whitespace = [char::from(9u8), char::from(160u8), char::from(10u8)]
        .into_iter()
        .collect::<String>();
    assert!([constants_str::EMPTY, constants_str::SPACE, whitespace.as_str()].into_iter().all(|text| {
        let reference = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(text);
        let presence = server_admin_core::std_admin_bool::StdAdminBool::from(false);
        optional_setting_matches::<server_admin_contract::admin_main_logo::AdminMainLogo>(reference, presence).get()
            && optional_setting_matches::<server_admin_contract::admin_organization_contacts::AdminOrganizationContacts>(reference, presence).get()
            && optional_setting_matches::<server_admin_contract::admin_organization_name::AdminOrganizationName>(reference, presence).get()
            && optional_setting_matches::<server_admin_contract::admin_primary_color::AdminPrimaryColor>(reference, presence).get()
            && optional_setting_matches::<server_admin_contract::admin_support_url::AdminSupportUrl>(reference, presence).get()
            && optional_setting_matches::<server_admin_contract::admin_tab_title::AdminTabTitle>(reference, presence).get()
    }));
    let nonblank = format!(" {} ", constants_str::ADMIN_ALT);
    let text = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(nonblank.as_str());
    let present = server_admin_core::std_admin_bool::StdAdminBool::from(true);
    assert!(
        optional_setting_matches::<
            server_admin_contract::admin_organization_contacts::AdminOrganizationContacts,
        >(text, present)
        .get()
    );
    assert!(
        optional_setting_matches::<
            server_admin_contract::admin_organization_name::AdminOrganizationName,
        >(text, present)
        .get()
    );
    assert!(
        optional_setting_matches::<server_admin_contract::admin_tab_title::AdminTabTitle>(
            text, present
        )
        .get()
    );
    let url = server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
        constants_str::HTTPS_EXAMPLE_COM,
    );
    assert!(
        optional_setting_matches::<server_admin_contract::admin_main_logo::AdminMainLogo>(
            url, present
        )
        .get()
    );
    assert!(
        optional_setting_matches::<server_admin_contract::admin_support_url::AdminSupportUrl>(
            url, present
        )
        .get()
    );
    assert!(
        optional_setting_matches::<server_admin_contract::admin_primary_color::AdminPrimaryColor>(
            server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
                constants_str::PRIMARY_COLOR_DEFAULT
            ),
            present,
        )
        .get()
    );
    assert!(
        crate::admin_html_form_text::AdminHtmlFormText::try_from(constants_str::X.to_owned())
            .is_ok_and(|form| {
                matches!(
                    crate::optional_setting_impl::optional_setting_impl::<
                        server_admin_contract::admin_support_url::AdminSupportUrl,
                        _,
                    >(form),
                    Err(crate::admin_error::AdminError::Validation)
                )
            })
    );
    assert!(
        crate::admin_html_form_text::AdminHtmlFormText::try_from(constants_str::X.to_owned())
            .is_ok_and(|form| {
                matches!(
                    crate::optional_setting_impl::optional_setting_impl::<
                        server_admin_contract::admin_primary_color::AdminPrimaryColor,
                        _,
                    >(form),
                    Err(crate::admin_error::AdminError::Validation)
                )
            })
    );
}

#[test]
fn test_refresh_token_context_hash_preserves_concatenation_order_and_secret_bounds() {
    let abc = [
        0xbau8, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22,
        0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00,
        0x15, 0xad,
    ];
    let cab = [
        0x65u8, 0x48, 0xd9, 0x55, 0x79, 0x0a, 0x22, 0x92, 0x5c, 0x1e, 0x23, 0x50, 0x8e, 0xc4, 0xe2,
        0xbf, 0xfb, 0x8e, 0x45, 0xd8, 0x02, 0x61, 0xb4, 0xb2, 0xc1, 0xf9, 0xd8, 0xc9, 0xb0, 0xd1,
        0x52, 0xb6,
    ];
    let empty = [
        0xe3u8, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];
    let bounded = [
        0xddu8, 0x4e, 0x67, 0x30, 0x52, 0x09, 0x32, 0x76, 0x7e, 0xc0, 0xa9, 0xe3, 0x3f, 0xe1, 0x9c,
        0x4c, 0xe2, 0x43, 0x99, 0xd6, 0xeb, 0xa4, 0xff, 0x62, 0xf1, 0x30, 0x13, 0xc9, 0xed, 0x30,
        0xef, 0x87,
    ];
    let cases = [
        (constants_str::AB.to_owned(), stringify!(c).to_owned(), abc),
        (stringify!(c).to_owned(), constants_str::AB.to_owned(), cab),
        (constants_str::ABC_ALT_3.to_owned(), String::new(), abc),
        (String::new(), constants_str::ABC_ALT_3.to_owned(), abc),
        (String::new(), String::new(), empty),
        (
            constants_str::A_ALT.repeat(8191usize),
            constants_str::A_ALT.to_owned(),
            bounded,
        ),
    ];
    assert!(cases.into_iter().all(|(token_text, context_text, expected)| {
        let converted = (
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(token_text),
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(context_text),
        );
        let (Ok(token_secret), Ok(context_secret)) = converted else { return false; };
        let token = crate::admin_opaque_token::AdminOpaqueToken::new(token_secret);
        let context = crate::admin_token_hash::AdminTokenHash::new(context_secret);
        crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(&token, &context)
            .is_ok_and(|hash| hash.expose().as_ref() == base16ct::lower::encode_string(&expected))
    }));
    let overflowing_token = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
        constants_str::A_ALT.repeat(8192usize),
    );
    let overflowing_context = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
        constants_str::A_ALT.to_owned(),
    );
    assert!(overflowing_token.is_ok_and(|token_secret| overflowing_context.is_ok_and(|context_secret| {
        matches!(
            crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
                &crate::admin_opaque_token::AdminOpaqueToken::new(token_secret),
                &crate::admin_token_hash::AdminTokenHash::new(context_secret),
            ),
            Err(crate::admin_secret_text_error::AdminSecretTextError::TooLong)
        )
    })));
}

#[tokio::test]
async fn test_html_response_preserves_content_type_and_exact_bounded_html_body() {
    let unicode = char::from(233u8).to_string();
    let checks = [
        constants_str::EMPTY,
        constants_str::ADMIN_DOCUMENT_UNSAFE_TITLE_FIXTURE,
        unicode.as_str(),
    ]
    .into_iter()
    .map(async |text| {
        let Ok(html) = frontend_admin::admin_ssr_html::AdminSsrHtml::try_from(text.to_owned())
        else {
            return false;
        };
        let response = crate::html_response_impl::html_response_impl(html);
        let expected_type = format!(
            "{}/{}; {}={}-{}",
            stringify!(text),
            stringify!(html),
            stringify!(charset),
            stringify!(utf),
            8u8
        );
        let headers_match = response.status() == http::StatusCode::OK
            && response
                .headers()
                .get(http::header::CONTENT_TYPE)
                .is_some_and(|header| header == expected_type.as_str());
        headers_match
            && axum::body::to_bytes(response.into_body(), 1024usize)
                .await
                .is_ok_and(|body| body.as_ref() == text.as_bytes())
    });
    assert!(
        futures::future::join_all(checks)
            .await
            .into_iter()
            .all(std::convert::identity)
    );
}

#[test]
fn test_user_path_conversion_preserves_positive_identifiers() {
    assert!([1i64, 7i64, i64::MAX].into_iter().all(|identifier| {
        server_admin_contract::admin_user_id::AdminUserId::try_from(identifier).is_ok_and(
            |admin_user_id| {
                let record = crate::user_path_impl::user_path_impl(admin_user_id);
                record.get() == identifier && record.value() == admin_user_id.value()
            },
        )
    }));
}
