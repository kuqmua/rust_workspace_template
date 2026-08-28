impl crate::AdminAuthSvcState {
    pub fn try_new(
        pool: app_state::domain_types::SqlxPgPool,
        jwt_secret: &config_lib::domain_types::AdminJwtSecret,
        access_ttl: &config_lib::domain_types::AdminAccessTokenTtlSeconds,
        refresh_ttl: &config_lib::domain_types::AdminRefreshTokenTtlSeconds,
        session_limit: &config_lib::domain_types::AdminSessionLimit,
        sign_in_rate_limit: &config_lib::domain_types::AdminSignInRateLimit,
        login_failure_limit: &config_lib::domain_types::AdminLoginFailureLimit,
        password_hash_concurrency: &config_lib::domain_types::AdminPasswordHashConcurrency,
        cookie_secure: &config_lib::domain_types::AdminCookieSecure,
        issuer: &config_lib::domain_types::AdminTokenIssuer,
        audience: &config_lib::domain_types::AdminTokenAudience,
        allowed_origins: &config_lib::domain_types::CorsAllowOrigin,
    ) -> Result<Self, crate::AdminAuthSvcStateBuildError> {
        let secret = secrecy::ExposeSecret::expose_secret(
            jwt_secret
                .primary()
                .ok_or(crate::AdminAuthSvcStateBuildError::JwtSecret)?
                .as_ref(),
        );
        let parsed_origins = allowed_origins
            .0
            .split(',')
            .map(str::trim)
            .map(str::to_owned)
            .collect::<Vec<String>>();
        Ok(Self {
            access_ttl: crate::StdAdminAccessTtlSeconds::try_from(access_ttl.get())
                .map_err(crate::AdminAuthSvcStateBuildError::PositiveValue)?,
            allowed_origins: server_runtime_http::domain_types::AllowedOrigins::try_from(
                parsed_origins,
            )
            .map_err(|_error| crate::AdminAuthSvcStateBuildError::AllowedOrigin)?,
            audience: audience.clone(),
            cookie_secure: crate::AdminCookieSecure::from(**cookie_secure),
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
            encoding_key: crate::JsonwebtokenAdminEncodingKey::from(
                jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            ),
            issuer: issuer.clone(),
            password_hasher: crate::AdminPasswordHasher::new(
                crate::AdminPasswordHashConcurrency::from(crate::AdminNonZeroUsize::from(
                    std::num::NonZeroUsize::new(password_hash_concurrency.get())
                        .ok_or(crate::AdminAuthSvcStateBuildError::PasswordHashConcurrency)?,
                )),
            ),
            pool,
            refresh_ttl: crate::StdAdminRefreshTtlSeconds::try_from(refresh_ttl.get())
                .map_err(crate::AdminAuthSvcStateBuildError::PositiveValue)?,
            session_limit: crate::StdAdminSessionLimit::try_from(session_limit.get())
                .map_err(crate::AdminAuthSvcStateBuildError::PositiveValue)?,
            policy: {
                let failure_threshold = crate::StdAdminFailureThreshold::try_from(
                    i64::try_from(login_failure_limit.get()).unwrap_or(i64::MAX),
                )
                .map_err(crate::AdminAuthSvcStateBuildError::PositiveValue)?;
                let sign_in_limit = crate::StdAdminRateLimitCount::from(
                    i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
                );
                crate::AdminAuthPolicy {
                    audit_export_limit: crate::StdAdminRateLimitCount::from(60i64),
                    audit_export_window: crate::StdAdminRateLimitWindowSeconds::from(60i32),
                    failure_delay: crate::StdAdminFailureDelayMillis::from(200u64),
                    failure_threshold,
                    mutation_limit: crate::StdAdminRateLimitCount::from(300i64),
                    mutation_window: crate::StdAdminRateLimitWindowSeconds::from(60i32),
                    refresh_limit: crate::StdAdminRateLimitCount::from(60i64),
                    refresh_window: crate::StdAdminRateLimitWindowSeconds::from(900i32),
                    sign_in_ip_limit: crate::StdAdminRateLimitCount::from(
                        sign_in_limit.0.saturating_mul(5i64),
                    ),
                    sign_in_limit,
                    sign_in_window: crate::StdAdminRateLimitWindowSeconds::from(900i32),
                }
            },
        })
    }
}
