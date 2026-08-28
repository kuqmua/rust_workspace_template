#[test]
fn cleanup_configuration_enforces_positive_bounded_values() {
    assert_eq!(
        crate::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
        Err(crate::AdminCleanupCfgError::BatchSizeOutOfRange)
    );
    assert_eq!(
        crate::AdminCleanupBatchSize::try_from(10_001i64),
        Err(crate::AdminCleanupCfgError::BatchSizeOutOfRange)
    );
    assert_eq!(
        crate::AdminCleanupRetentionSeconds::try_from(constants_i64::ZERO),
        Err(crate::AdminCleanupCfgError::RetentionMustBePositive)
    );
    assert_eq!(
        crate::AdminCleanupBatchSize::try_from(1_000i64).map(|_value| ()),
        Ok(())
    );
    assert_eq!(
        crate::AdminCleanupRetentionSeconds::try_from(3_600i64).map(|_value| ()),
        Ok(())
    );
}
fn admin_secret(value: &str) -> crate::SecrecyAdminString {
    crate::SecrecyAdminString::try_from(value.to_owned())
        .expect("c2116874 secret invariant must hold")
}
fn password(value: &str) -> crate::AdminPassword {
    crate::AdminPassword::new(admin_secret(value))
}
#[test]
fn permission_round_trip_is_exhaustive() {
    crate::AdminPermission::ALL
        .into_iter()
        .for_each(|permission| {
            assert_eq!(
                crate::AdminPermission::try_from(permission.as_str().as_ref())
                    .expect("0f53b75c permission_round_trip_is_exhaustive invariant must hold"),
                permission
            );
        });
}
#[test]
fn permission_serializes_as_public_contract_value() {
    assert_eq!(
        serde_json::to_string(&crate::AdminPermission::UsersRead)
            .expect("9a6b413e permission_serializes_as_public_contract_value invariant must hold"),
        "\"users:read\""
    );
}
#[test]
fn unknown_permission_is_rejected() {
    assert_eq!(
        crate::AdminPermission::try_from(constants_str::UNKNOWN_READ).err(),
        Some(crate::AdminPermissionTryFromStrError)
    );
}
#[test]
fn migration_inventory_is_not_empty() {
    let migrator = crate::migrations::migrator();
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
    assert!(crate::AdminPermission::ALL.into_iter().all(|permission| {
        crate::migrations::migrator().iter().any(|migration| {
            migration
                .sql
                .as_str()
                .contains(permission.as_str().as_ref())
        })
    }));
}
#[tokio::test]
async fn password_hash_verifies_only_matching_password() {
    let hasher = crate::AdminPasswordHasher::new(crate::AdminPasswordHashConcurrency::from(
        std::num::NonZeroUsize::new(1)
            .expect("70761471 password hash test concurrency invariant must hold"),
    ));
    let hash = hasher
        .hash(password(constants_str::CORRECT_PASSWORD_ALT))
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
        .hash(password(constants_str::CORRECT_PASSWORD_ALT))
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
    let raw_secret = constants_str::NEVER_PRINT_THIS_VALUE;
    let password = password(raw_secret);
    let jwt_secret = crate::AdminJwtSecret::new(admin_secret(raw_secret));
    let access_token = crate::StdAdminAccessToken::try_from(raw_secret.to_owned())
        .expect("e295277c secrets_are_redacted_in_debug_output invariant must hold");
    assert!(!format!("{password:?}").contains(raw_secret));
    assert!(!format!("{jwt_secret:?}").contains(raw_secret));
    assert!(!format!("{access_token:?}").contains(raw_secret));
}
#[test]
fn generated_token_hash_is_stable_and_does_not_expose_token() {
    let token = crate::AdminOpaqueToken::new(admin_secret(constants_str::FIXED_TEST_TOKEN));
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
    let access = crate::build_admin_cookie(
        crate::AdminCookieKind::Access,
        crate::StdAdminStrRef::from(constants_str::ACCESS),
        crate::AdminCookieMaxAgeSeconds::from(60),
        crate::AdminCookieSecure::from(true),
    );
    let csrf = crate::build_admin_cookie(
        crate::AdminCookieKind::Csrf,
        crate::StdAdminStrRef::from(constants_str::CSRF),
        crate::AdminCookieMaxAgeSeconds::from(60),
        crate::AdminCookieSecure::from(true),
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
            constants_str::OTHER_1_ADMIN_ACCESS_TOKEN_EXPECTED_ADMIN_ACCESS_TOKEN_SUFFIX_WRONG,
        ),
    );
    assert_eq!(
        crate::find_admin_cookie(
            crate::HttpAdminHeaderMapRef::from(&headers),
            crate::AdminCookieKind::Access,
        ),
        Some(crate::StdAdminStrRef::from("expected"))
    );
}
#[test]
fn administrator_login_format_accepts_only_database_compatible_values() {
    let valid =
        crate::AdminLogin::try_from(constants_str::ADMIN_USER_1.to_owned()).expect("078c759d administrator_login_format_accepts_only_database_compatible_values invariant must hold");
    assert_eq!(valid.as_ref(), constants_str::ADMIN_USER_1);
    let _uppercase_error = crate::AdminLogin::try_from(constants_str::ADMIN.to_owned())
        .expect_err(constants_str::VALUE_5FA1C6E2);
    let _short_error = crate::AdminLogin::try_from(constants_str::AB.to_owned())
        .expect_err(constants_str::VALUE_B78D42A9);
}
#[test]
fn access_token_round_trip_checks_issuer_and_audience() {
    let claims = crate::AdminAccessClaims::new(
        crate::AdminUserId::try_from(7).expect(
            "d6d3da8a access_token_round_trip_checks_issuer_and_audience invariant must hold",
        ),
        crate::AdminSessionId::from(crate::UuidAdminValue::from(
            uuid::Uuid::parse_str(constants_str::B871BD8F_7810_4D4B_94A1_5458D3016907).expect(
                "05562da0 access_token_round_trip_checks_issuer_and_audience invariant must hold",
            ),
        )),
        crate::AdminUnixTokenStream::from(1),
        crate::AdminUnixTokenStream::from(4_102_444_800),
        config_lib::domain_types::AdminTokenIssuer::try_from(constants_str::TEST_ISSUER.to_owned())
            .expect(
                "fd6a65b0 access_token_round_trip_checks_issuer_and_audience invariant must hold",
            ),
        config_lib::domain_types::AdminTokenAudience::try_from(
            constants_str::TEST_AUDIENCE.to_owned(),
        )
        .expect("6e423e16 access_token_round_trip_checks_issuer_and_audience invariant must hold"),
    );
    let secret = crate::AdminJwtSecret::new(admin_secret(
        constants_str::TEST_ONLY_SECRET_WITH_SUFFICIENT_ENTROPY,
    ));
    let token = crate::encode_access_token(&claims, &secret)
        .expect("b41052bc access_token_round_trip_checks_issuer_and_audience invariant must hold");
    let issuer =
        config_lib::domain_types::AdminTokenIssuer::try_from(constants_str::TEST_ISSUER.to_owned())
            .expect(
                "5edc807f access_token_round_trip_checks_issuer_and_audience invariant must hold",
            );
    let audience = config_lib::domain_types::AdminTokenAudience::try_from(
        constants_str::TEST_AUDIENCE.to_owned(),
    )
    .expect("0c3975a1 access_token_round_trip_checks_issuer_and_audience invariant must hold");
    let decoded = crate::decode_access_token(&token, &secret, &issuer, &audience)
        .expect("0ed905ff access_token_round_trip_checks_issuer_and_audience invariant must hold");
    assert_eq!(
        decoded.user_id(),
        crate::AdminUserId::try_from(7).expect(
            "5b88f22a access_token_round_trip_checks_issuer_and_audience invariant must hold"
        )
    );
    assert_eq!(decoded.session_id(), claims.session_id());
    drop(
        crate::decode_access_token(
            &token,
            &secret,
            &issuer,
            &config_lib::domain_types::AdminTokenAudience::try_from(
                constants_str::WRONG_AUDIENCE.to_owned(),
            )
            .expect(
                "92f9c5ec access_token_round_trip_checks_issuer_and_audience invariant must hold",
            ),
        )
        .expect_err(constants_str::A82438CC),
    );
}
