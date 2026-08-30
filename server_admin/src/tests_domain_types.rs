#[test]
fn cleanup_configuration_enforces_positive_bounded_values() {
    assert_eq!(
        crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
        Err(crate::admin_cleanup_cfg_error::AdminCleanupCfgError::BatchSizeOutOfRange)
    );
    assert_eq!(
        crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(10_001i64),
        Err(crate::admin_cleanup_cfg_error::AdminCleanupCfgError::BatchSizeOutOfRange)
    );
    assert_eq!(
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(
            constants_i64::ZERO
        ),
        Err(crate::admin_cleanup_cfg_error::AdminCleanupCfgError::RetentionMustBePositive)
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
fn admin_secret(value: &str) -> server_admin_core::secrecy_admin_string::SecrecyAdminString {
    server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(value.to_owned())
        .expect("c2116874 secret invariant must hold")
}
fn password(value: &str) -> crate::runtime_admin_password::RuntimeAdminPassword {
    crate::runtime_admin_password::RuntimeAdminPassword::new(admin_secret(value))
}
#[test]
fn permission_round_trip_is_exhaustive() {
    server_admin_contract::admin_permission::AdminPermission::ALL
        .into_iter()
        .for_each(|permission| {
            assert_eq!(
                server_admin_contract::admin_permission::AdminPermission::try_from(
                    permission.as_str().as_ref()
                )
                .expect("0f53b75c permission_round_trip_is_exhaustive invariant must hold"),
                permission
            );
        });
}
#[test]
fn permission_serializes_as_public_contract_value() {
    assert_eq!(
        serde_json::to_string(&server_admin_contract::admin_permission::AdminPermission::UsersRead)
            .expect("9a6b413e permission_serializes_as_public_contract_value invariant must hold"),
        "\"users:read\""
    );
}
#[test]
fn unknown_permission_is_rejected() {
    assert_eq!(
        server_admin_contract::admin_permission::AdminPermission::try_from(
            constants_str::catalog::UNKNOWN_READ
        )
        .err(),
        Some(server_admin_contract::admin_permission::AdminPermissionTryFromStrError)
    );
}
#[test]
fn migration_inventory_is_not_empty() {
    let migrator = crate::migrator::migrator();
    let migrations = migrator.iter().collect::<Vec<_>>();
    assert_eq!(migrations.len(), 13usize);
    assert!(
        migrations
            .iter()
            .any(|migration| migration.description == "admin schema")
    );
}
#[test]
fn permission_seed_contains_the_complete_typed_catalog() {
    assert!(
        server_admin_contract::admin_permission::AdminPermission::ALL
            .into_iter()
            .all(|permission| {
                crate::migrator::migrator().iter().any(|migration| {
                    migration
                        .sql
                        .as_str()
                        .contains(permission.as_str().as_ref())
                })
            })
    );
}
#[tokio::test]
async fn password_hash_verifies_only_matching_password() {
    let hasher = crate::admin_password_hasher::AdminPasswordHasher::new(
        crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
            std::num::NonZeroUsize::new(1)
                .expect("70761471 password hash test concurrency invariant must hold"),
        ),
    );
    let hash = hasher
        .hash(password(constants_str::catalog::CORRECT_PASSWORD_ALT))
        .await
        .expect("174a5d2f password_hash_verifies_only_matching_password invariant must hold");
    assert!(
        hasher
            .verify(password("correct password"), hash)
            .await
            .expect("604f40be password_hash_verifies_only_matching_password invariant must hold")
            .get()
    );
    let other_hash = hasher
        .hash(password(constants_str::catalog::CORRECT_PASSWORD_ALT))
        .await
        .expect("38819b94 password_hash_verifies_only_matching_password invariant must hold");
    assert!(
        !hasher
            .verify(password("wrong password"), other_hash)
            .await
            .expect("ed6b499a password_hash_verifies_only_matching_password invariant must hold")
            .get()
    );
}
#[test]
fn secrets_are_redacted_in_debug_output() {
    let raw_secret = constants_str::catalog::NEVER_PRINT_THIS_VALUE;
    let password = password(raw_secret);
    let jwt_secret =
        crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret::new(admin_secret(raw_secret));
    let access_token =
        crate::std_admin_access_token::StdAdminAccessToken::try_from(raw_secret.to_owned())
            .expect("e295277c secrets_are_redacted_in_debug_output invariant must hold");
    assert!(!format!("{password:?}").contains(raw_secret));
    assert!(!format!("{jwt_secret:?}").contains(raw_secret));
    assert!(!format!("{access_token:?}").contains(raw_secret));
}
#[test]
fn generated_token_hash_is_stable_and_does_not_expose_token() {
    let token = crate::admin_opaque_token::AdminOpaqueToken::new(admin_secret(
        constants_str::catalog::FIXED_TEST_TOKEN,
    ));
    let hash = crate::hash_opaque_token::hash_opaque_token(&token).expect(
        "3af32394 generated_token_hash_is_stable_and_does_not_expose_token invariant must hold",
    );
    assert_eq!(
        hash.expose().as_ref(),
        "abae2c734c2b0249ef1d413fdf30c332c6875fde570f9bbeef4295966f0b4943"
    );
    assert!(!format!("{hash:?}").contains("fixed-test-token"));
}
#[test]
fn cookie_policy_marks_only_secret_tokens_http_only() {
    let access = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Access,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::catalog::ACCESS),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(60),
        crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(true),
    );
    let csrf = crate::build_admin_cookie::build_admin_cookie(
        crate::admin_cookie_kind::AdminCookieKind::Csrf,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::catalog::CSRF),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(60),
        crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(true),
    );
    assert!(access.as_ref().contains("HttpOnly"));
    assert!(access.as_ref().contains("Secure"));
    assert!(access.as_ref().contains("SameSite=Strict"));
    assert!(!csrf.as_ref().contains("HttpOnly"));
    assert!(csrf.as_ref().contains("Secure"));
}
#[test]
fn cookie_parser_matches_complete_cookie_name() {
    let mut headers = http::HeaderMap::new();
    let _previous = headers.insert(
        http::header::COOKIE,
        http::HeaderValue::from_static(
            constants_str::catalog::OTHER_1_ADMIN_ACCESS_TOKEN_EXPECTED_ADMIN_ACCESS_TOKEN_SUFFIX_WRONG,
        ),
    );
    assert_eq!(
        crate::find_admin_cookie::find_admin_cookie(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
            crate::admin_cookie_kind::AdminCookieKind::Access,
        ),
        Some(server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            "expected"
        ))
    );
}
#[test]
fn administrator_login_format_accepts_only_database_compatible_values() {
    let valid =
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::catalog::ADMIN_USER_1.to_owned()).expect("078c759d administrator_login_format_accepts_only_database_compatible_values invariant must hold");
    assert_eq!(valid.as_ref(), constants_str::catalog::ADMIN_USER_1);
    let _uppercase_error = server_admin_contract::admin_login::AdminLogin::try_from(
        constants_str::catalog::ADMIN.to_owned(),
    )
    .expect_err(constants_str::test_fixtures::VALUE_5FA1C6E2);
    let _short_error = server_admin_contract::admin_login::AdminLogin::try_from(
        constants_str::catalog::AB.to_owned(),
    )
    .expect_err(constants_str::test_fixtures::VALUE_B78D42A9);
}
#[test]
fn access_token_round_trip_checks_issuer_and_audience() {
    let claims = crate::admin_access_claims::AdminAccessClaims::new(
        server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(7).expect(
            "d6d3da8a access_token_round_trip_checks_issuer_and_audience invariant must hold",
        ),
        crate::admin_session_id::AdminSessionId::from(server_admin_core::uuid_admin_value::UuidAdminValue::from(
            uuid::Uuid::parse_str(constants_str::catalog::B871BD8F_7810_4D4B_94A1_5458D3016907).expect(
                "05562da0 access_token_round_trip_checks_issuer_and_audience invariant must hold",
            ),
        )),
        crate::admin_unix_token_stream::AdminUnixTokenStream::from(1),
        crate::admin_unix_token_stream::AdminUnixTokenStream::from(4_102_444_800),
        config_lib::admin_token_issuer::AdminTokenIssuer::try_from(constants_str::catalog::TEST_ISSUER.to_owned())
            .expect(
                "fd6a65b0 access_token_round_trip_checks_issuer_and_audience invariant must hold",
            ),
        config_lib::admin_token_audience::AdminTokenAudience::try_from(
            constants_str::catalog::TEST_AUDIENCE.to_owned(),
        )
        .expect("6e423e16 access_token_round_trip_checks_issuer_and_audience invariant must hold"),
    );
    let secret = crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret::new(admin_secret(
        constants_str::catalog::TEST_ONLY_SECRET_WITH_SUFFICIENT_ENTROPY,
    ));
    let token = crate::encode_access_token::encode_access_token(&claims, &secret)
        .expect("b41052bc access_token_round_trip_checks_issuer_and_audience invariant must hold");
    let issuer = config_lib::admin_token_issuer::AdminTokenIssuer::try_from(
        constants_str::catalog::TEST_ISSUER.to_owned(),
    )
    .expect("5edc807f access_token_round_trip_checks_issuer_and_audience invariant must hold");
    let audience = config_lib::admin_token_audience::AdminTokenAudience::try_from(
        constants_str::catalog::TEST_AUDIENCE.to_owned(),
    )
    .expect("0c3975a1 access_token_round_trip_checks_issuer_and_audience invariant must hold");
    let decoded =
        crate::decode_access_token::decode_access_token(&token, &secret, &issuer, &audience)
            .expect(
                "0ed905ff access_token_round_trip_checks_issuer_and_audience invariant must hold",
            );
    assert_eq!(
        decoded.user_id(),
        server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(7).expect(
            "5b88f22a access_token_round_trip_checks_issuer_and_audience invariant must hold"
        )
    );
    assert_eq!(decoded.session_id(), claims.session_id());
    drop(
        crate::decode_access_token::decode_access_token(
            &token,
            &secret,
            &issuer,
            &config_lib::admin_token_audience::AdminTokenAudience::try_from(
                constants_str::catalog::WRONG_AUDIENCE.to_owned(),
            )
            .expect(
                "92f9c5ec access_token_round_trip_checks_issuer_and_audience invariant must hold",
            ),
        )
        .expect_err(constants_str::catalog::A82438CC),
    );
}
