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
            .get_inner()
            .split(',')
            .map(str::trim)
            .map(str::to_owned)
            .collect::<Vec<String>>();
        Ok(Self::new(
            crate::std_admin_access_ttl_seconds::StdAdminAccessTtlSeconds::try_from(access_ttl.get())
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?,
            server_runtime_http::allowed_origins::AllowedOrigins::try_from(
                parsed_origins,
            )
            .map_err(|_error| crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::AllowedOrigin)?,
            audience.clone(),
            jwt_secret
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
            crate::jsonwebtoken_admin_encoding_key::JsonwebtokenAdminEncodingKey::from(
                jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            ),
            issuer.clone(),
            crate::admin_password_hasher::AdminPasswordHasher::new(
                crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
                    std::num::NonZeroUsize::new(password_hash_concurrency.get())
                        .ok_or(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PasswordHashConcurrency)?,
                ),
            ),
            {
                let failure_threshold = crate::std_admin_failure_threshold::StdAdminFailureThreshold::try_from(
                    i64::try_from(login_failure_limit.get()).unwrap_or(i64::MAX),
                )
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?;
                let sign_in_limit = crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(
                    i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
                );
                crate::admin_auth_policy::AdminAuthPolicy::new(
                    crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(60i64),
                    crate::std_admin_failure_delay_millis::StdAdminFailureDelayMillis::from(200u64),
                    failure_threshold,
                    crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(300i64),
                    crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(60i64),
                    crate::std_admin_rate_limit_count::StdAdminRateLimitCount::from(
                        sign_in_limit.get_inner().saturating_mul(5i64),
                    ),
                    sign_in_limit,
                    crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(60i32),
                    crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(60i32),
                    crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(900i32),
                    crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds::from(900i32),
                )
            },
            pool,
            crate::std_admin_refresh_ttl_seconds::StdAdminRefreshTtlSeconds::try_from(refresh_ttl.get())
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?,
            crate::std_admin_session_limit::StdAdminSessionLimit::try_from(session_limit.get())
                .map_err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::PositiveValue)?,
            crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure::from(**cookie_secure),
        ))
    }
}
