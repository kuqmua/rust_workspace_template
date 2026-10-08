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
        crate::admin_session_id::AdminSessionId::from(
            server_admin_core::uuid_admin_value::UuidAdminValue::from(
                uuid::Uuid::parse_str(constants_str::B871BD8F_7810_4D4B_94A1_5458D3016907)
                    .expect(constants_str::DIAGNOSTIC_05562DA0),
            ),
        ),
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
