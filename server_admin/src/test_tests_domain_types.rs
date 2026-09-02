#[test]
fn test_cleanup_configuration_enforces_positive_bounded_values() {
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
        .expect(constants_str::DIAGNOSTIC_C2116874)
}
fn password(value: &str) -> crate::runtime_admin_password::RuntimeAdminPassword {
    crate::runtime_admin_password::RuntimeAdminPassword::new(admin_secret(value))
}
#[test]
fn test_permission_round_trip_is_exhaustive() {
    server_admin_contract::admin_permission::AdminPermission::ALL
        .into_iter()
        .for_each(|permission| {
            assert_eq!(
                server_admin_contract::admin_permission::AdminPermission::try_from(
                    permission.as_str().as_ref()
                )
                .expect(constants_str::DIAGNOSTIC_0F53B75C),
                permission
            );
        });
}
#[test]
fn test_permission_serializes_as_public_contract_value() {
    assert_eq!(
        serde_json::to_string(&server_admin_contract::admin_permission::AdminPermission::UsersRead)
            .expect(constants_str::DIAGNOSTIC_9A6B413E),
        constants_str::VALUE_6E7831EE
    );
}
#[test]
fn test_unknown_permission_is_rejected() {
    assert_eq!(
        server_admin_contract::admin_permission::AdminPermission::try_from(
            constants_str::UNKNOWN_READ
        )
        .err(),
        Some(server_admin_contract::admin_permission::AdminPermissionTryFromStrError)
    );
}
#[test]
fn test_migration_inventory_is_not_empty() {
    let migrator = crate::migrator::migrator();
    let migrations = migrator.iter().collect::<Vec<_>>();
    assert_eq!(migrations.len(), 13usize);
    assert!(
        migrations
            .iter()
            .any(|migration| migration.description == constants_str::VALUE_6A6C872E)
    );
}
#[test]
fn test_permission_seed_contains_the_complete_typed_catalog() {
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
    assert!(access.as_ref().contains(constants_str::VALUE_A0820391));
    assert!(access.as_ref().contains(constants_str::VALUE_1BCED1D0));
    assert!(access.as_ref().contains(constants_str::VALUE_DD7C3F04));
    assert!(!csrf.as_ref().contains(constants_str::VALUE_A0820391));
    assert!(csrf.as_ref().contains(constants_str::VALUE_1BCED1D0));
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
    drop(
        crate::decode_access_token::decode_access_token(
            &token,
            &secret,
            &issuer,
            &config_lib::admin_token_audience::AdminTokenAudience::try_from(
                constants_str::WRONG_AUDIENCE.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_92F9C5EC),
        )
        .expect_err(constants_str::A82438CC),
    );
}
