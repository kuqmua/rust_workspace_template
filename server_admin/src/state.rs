impl crate::admin_auth_svc_state::AdminAuthSvcState {
    pub fn try_new(
        pool: app_state::sqlx_pg_pool::SqlxPgPool,
        jwt_secret: &config_lib::admin_jwt_secret::AdminJwtSecret,
        access_ttl: &config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds,
        refresh_ttl: &config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds,
        session_limit: &config_lib::admin_session_limit::AdminSessionLimit,
        sign_in_rate_limit: &config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit,
        login_failure_limit: &config_lib::admin_login_failure_limit::AdminLoginFailureLimit,
        password_hash_concurrency: &config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency,
        cookie_secure: &config_lib::admin_cookie_secure::AdminCookieSecure,
        issuer: &config_lib::admin_token_issuer::AdminTokenIssuer,
        audience: &config_lib::admin_token_audience::AdminTokenAudience,
        allowed_origins: &config_lib::domain_types::CorsAllowOrigin,
    ) -> Result<Self, crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError> {
        let secret = secrecy::ExposeSecret::expose_secret(
            jwt_secret
                .primary()
                .ok_or(
                    crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::JwtSecret,
                )?
                .as_ref(),
        );
        let parsed_origins = allowed_origins
            .0
            .split(',')
            .map(str::trim)
            .map(str::to_owned)
            .collect::<Vec<String>>();
        Ok(Self {
            access_ttl: crate::std_admin_access_ttl_seconds::StdAdminAccessTtlSeconds::try_from(access_ttl.get())
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?,
            allowed_origins: server_runtime_http::allowed_origins::AllowedOrigins::try_from(
                parsed_origins,
            )
            .map_err(|_error| crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::AllowedOrigin)?,
            audience: audience.clone(),
            cookie_secure: crate::admin_cookie_secure::AdminCookieSecure::from(**cookie_secure),
            decoding_keys: jwt_secret
                .verification_secrets()
                .iter()
                .map(|verification_secret| {
                    jsonwebtoken::DecodingKey::from_secret(
                        secrecy::ExposeSecret::expose_secret(verification_secret.as_ref())
                            .as_bytes(),
                    )
                })
                .collect::<Vec<_>>()
                .into(),
            encoding_key: crate::jsonwebtoken_admin_encoding_key::JsonwebtokenAdminEncodingKey::from(
                jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            ),
            issuer: issuer.clone(),
            password_hasher: crate::admin_password_hasher::AdminPasswordHasher::new(
                crate::admin_password_hash_concurrency::AdminPasswordHashConcurrency::from(
                    std::num::NonZeroUsize::new(password_hash_concurrency.get())
                        .ok_or(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PasswordHashConcurrency)?,
                ),
            ),
            pool,
            refresh_ttl: crate::std_admin_refresh_ttl_seconds::StdAdminRefreshTtlSeconds::try_from(refresh_ttl.get())
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?,
            session_limit: crate::std_admin_session_limit::StdAdminSessionLimit::try_from(session_limit.get())
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?,
            policy: {
                let failure_threshold = crate::std_admin_failure_threshold::StdAdminFailureThreshold::try_from(
                    i64::try_from(login_failure_limit.get()).unwrap_or(i64::MAX),
                )
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?;
                let sign_in_limit = crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(
                    i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
                );
                crate::admin_auth_policy::AdminAuthPolicy {
                    audit_export_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(60i64),
                    audit_export_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(60i32),
                    failure_delay: crate::std_admin_failure_delay_millis::StdAdminFailureDelayMillis::from(200u64),
                    failure_threshold,
                    mutation_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(300i64),
                    mutation_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(60i32),
                    refresh_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(60i64),
                    refresh_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(900i32),
                    sign_in_ip_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(
                        sign_in_limit.0.saturating_mul(5i64),
                    ),
                    sign_in_limit,
                    sign_in_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(900i32),
                }
            },
        })
    }
}
