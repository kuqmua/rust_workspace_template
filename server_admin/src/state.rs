impl super::AdminAuthSvcState {
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
    ) -> Result<Self, super::AdminAuthSvcStateBuildError> {
        let secret = secrecy::ExposeSecret::expose_secret(
            jwt_secret
                .primary()
                .ok_or(super::AdminAuthSvcStateBuildError::JwtSecret)?
                .as_ref(),
        );
        let parsed_origins = allowed_origins
            .0
            .split(',')
            .map(str::trim)
            .map(str::to_owned)
            .collect::<Vec<String>>();
        Ok(Self {
            access_ttl: super::StdAdminAccessTtlSeconds::try_from(access_ttl.get())
                .map_err(super::AdminAuthSvcStateBuildError::PositiveValue)?,
            allowed_origins: server_runtime_http::domain_types::AllowedOrigins::try_from(
                parsed_origins,
            )
            .map_err(|_error| super::AdminAuthSvcStateBuildError::AllowedOrigin)?,
            audience: audience.clone(),
            cookie_secure: super::super::AdminCookieSecure::from(**cookie_secure),
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
            encoding_key: super::JsonwebtokenAdminEncodingKey::from(
                jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            ),
            issuer: issuer.clone(),
            password_hasher: super::super::AdminPasswordHasher::new(
                super::super::AdminPasswordHashConcurrency::from(
                    super::super::AdminNonZeroUsize::from(
                        std::num::NonZeroUsize::new(password_hash_concurrency.get())
                            .ok_or(super::AdminAuthSvcStateBuildError::PasswordHashConcurrency)?,
                    ),
                ),
            ),
            pool,
            refresh_ttl: super::StdAdminRefreshTtlSeconds::try_from(refresh_ttl.get())
                .map_err(super::AdminAuthSvcStateBuildError::PositiveValue)?,
            session_limit: super::StdAdminSessionLimit::try_from(session_limit.get())
                .map_err(super::AdminAuthSvcStateBuildError::PositiveValue)?,
            policy: {
                let failure_threshold = super::StdAdminFailureThreshold::try_from(
                    i64::try_from(login_failure_limit.get()).unwrap_or(i64::MAX),
                )
                .map_err(super::AdminAuthSvcStateBuildError::PositiveValue)?;
                let sign_in_limit = super::StdAdminRateLimitCount::from(
                    i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
                );
                super::AdminAuthPolicy {
                    audit_export_limit: super::StdAdminRateLimitCount::from(60i64),
                    audit_export_window: super::StdAdminRateLimitWindowSeconds::from(60i32),
                    failure_delay: super::StdAdminFailureDelayMillis::from(200u64),
                    failure_threshold,
                    mutation_limit: super::StdAdminRateLimitCount::from(300i64),
                    mutation_window: super::StdAdminRateLimitWindowSeconds::from(60i32),
                    refresh_limit: super::StdAdminRateLimitCount::from(60i64),
                    refresh_window: super::StdAdminRateLimitWindowSeconds::from(900i32),
                    sign_in_ip_limit: super::StdAdminRateLimitCount::from(
                        sign_in_limit.0.saturating_mul(5i64),
                    ),
                    sign_in_limit,
                    sign_in_window: super::StdAdminRateLimitWindowSeconds::from(900i32),
                }
            },
        })
    }
}

#[cfg(test)]
#[path = "application__state__tests.rs"]
mod tests;
