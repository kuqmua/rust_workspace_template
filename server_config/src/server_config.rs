// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, try_from_env::TryFromEnv, optimal_memory_layout::OptimalMemoryLayout)]
#[config(env_example)]
pub struct ServerConfig {
    #[config(example = "http://127.0.0.1:8080")]
    pub cors_allow_origin: config_lib::domain_types::CorsAllowOrigin,
    #[config(
        example = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; object-src 'none'; frame-ancestors 'none'"
    )]
    pub content_security_policy: config_lib::content_security_policy::ContentSecurityPolicy,
    #[config(secret)]
    #[config(example = "postgres://postgres:change-me@127.0.0.1:5432/rust_workspace_template")]
    pub database_url: config_lib::domain_types::DatabaseUrl,
    #[config(secret)]
    #[config(example = "change-me-development-secret-000")]
    pub admin_jwt_secret: config_lib::admin_jwt_secret::AdminJwtSecret,
    #[config(example = "rust-workspace-template")]
    pub admin_token_audience: config_lib::admin_token_audience::AdminTokenAudience,
    #[config(example = "rust-workspace-template")]
    pub admin_token_issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
    #[config(example = "127.0.0.1/32,::1/128")]
    pub trusted_proxy_ranges_text: config_lib::domain_types::TrustedProxyRangesText,
    #[config(example = "900")]
    pub admin_access_token_ttl_seconds:
        config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds,
    #[config(example = "2")]
    pub admin_password_hash_concurrency:
        config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency,
    #[config(example = "10")]
    pub admin_login_failure_limit: config_lib::admin_login_failure_limit::AdminLoginFailureLimit,
    #[config(example = "604800")]
    pub admin_refresh_token_ttl_seconds:
        config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds,
    #[config(example = "8")]
    pub admin_session_limit: config_lib::admin_session_limit::AdminSessionLimit,
    #[config(example = "10")]
    pub admin_sign_in_rate_limit: config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit,
    #[config(example = "10")]
    pub pg_pool_acquire_timeout_seconds:
        config_lib::pg_pool_acquire_timeout_seconds::PgPoolAcquireTimeoutSeconds,
    #[config(example = "600")]
    pub pg_pool_idle_timeout_seconds:
        config_lib::pg_pool_idle_timeout_seconds::PgPoolIdleTimeoutSeconds,
    #[config(example = "1800")]
    pub pg_pool_max_lifetime_seconds:
        config_lib::pg_pool_max_lifetime_seconds::PgPoolMaxLifetimeSeconds,
    #[config(example = "30")]
    pub request_timeout_seconds: config_lib::request_timeout_seconds::RequestTimeoutSeconds,
    #[config(example = "1048576")]
    pub maximum_size_of_http_body_in_bytes:
        config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes,
    #[config(example = "127.0.0.1:8080")]
    pub service_socket_address: config_lib::domain_types::ServiceSocketAddress,
    #[config(example = "10")]
    pub pg_pool_max_connections: config_lib::pg_pool_max_connections::PgPoolMaxConnections,
    #[config(example = "1")]
    pub pg_pool_min_connections: config_lib::pg_pool_min_connections::PgPoolMinConnections,
    #[config(example = "10800")]
    pub timezone: config_lib::chrono_timezone::ChronoTimezone,
    #[config(example = "src")]
    pub src_place_type: config_lib::domain_types::SrcPlaceType,
    #[config(example = "info")]
    pub tracing_level: config_lib::domain_types::TracingLevel,
    #[config(example = "text")]
    pub tracing_format: config_lib::tracing_format::TracingFormat,
    #[config(example = "true")]
    pub enable_api_git_commit_check: config_lib::domain_types::EnableApiGitCommitCheck,
    #[config(example = "false")]
    pub admin_cookie_secure: config_lib::admin_cookie_secure::AdminCookieSecure,
    #[config(example = "true")]
    pub admin_swagger_enabled: config_lib::admin_swagger_enabled::AdminSwaggerEnabled,
    #[config(example = "true")]
    pub http_gzip_enabled: config_lib::http_gzip_enabled::HttpGzipEnabled,
    #[config(example = "false")]
    pub production_mode: config_lib::production_mode::ProductionMode,
    #[config(example = "serve")]
    pub svc_mode: config_lib::svc_mode::SvcMode,
}
impl ServerConfig {
    pub fn validate_for_startup(
        &self,
    ) -> Result<(), crate::production_config_error::ProductionConfigError> {
        if !*self.production_mode {
            return Ok(());
        }
        if !*self.admin_cookie_secure {
            return Err(crate::production_config_error::ProductionConfigError::AdminCookieInsecure);
        }
        if *self.admin_swagger_enabled {
            return Err(crate::production_config_error::ProductionConfigError::AdminSwaggerEnabled);
        }
        if !self
            .cors_allow_origin
            .0
            .as_str()
            .split(',')
            .map(str::trim)
            .all(|origin| {
                !origin.is_empty() && origin.starts_with(constants_str::HTTPS_SCHEME_PREFIX)
            })
        {
            return Err(crate::production_config_error::ProductionConfigError::CorsOriginInsecure);
        }
        if self
            .admin_jwt_secret
            .verification_secrets()
            .iter()
            .any(|secret| {
                secrecy::ExposeSecret::expose_secret(secret.as_ref()).as_ref()
                    == constants_str::ADMIN_DEVELOPMENT_JWT_SECRET
            })
        {
            return Err(
                crate::production_config_error::ProductionConfigError::DevelopmentJwtSecret,
            );
        }
        Ok(())
    }
}
impl config_lib::domain_types::CorsAllowOriginProvider for ServerConfig {
    fn cors_allow_origin(&self) -> &String {
        &self.cors_allow_origin.0
    }
}
impl config_lib::domain_types::DatabaseUrlProvider for ServerConfig {
    fn database_url(
        &self,
    ) -> &secrecy::SecretBox<config_lib::std_config_secret_string::StdConfigSecretString> {
        &self.database_url.0
    }
}
impl config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytesProvider
    for ServerConfig
{
    fn maximum_size_of_http_body_in_bytes(&self) -> &usize {
        &self.maximum_size_of_http_body_in_bytes
    }
}
impl config_lib::domain_types::ServiceSocketAddressProvider for ServerConfig {
    fn service_socket_address(&self) -> &std::net::SocketAddr {
        &self.service_socket_address.0
    }
}
impl config_lib::pg_pool_max_connections::PgPoolMaxConnectionsProvider for ServerConfig {
    fn pg_pool_max_connections(&self) -> &u32 {
        &self.pg_pool_max_connections
    }
}
impl config_lib::chrono_timezone::ChronoTimezoneProvider for ServerConfig {
    fn chrono_timezone(&self) -> &chrono::FixedOffset {
        &self.timezone
    }
}
impl config_lib::domain_types::SrcPlaceTypeProvider for ServerConfig {
    fn src_place_type(&self) -> &config_lib::src_place_type::SrcPlaceType {
        &self.src_place_type.0
    }
}
impl config_lib::domain_types::TracingLevelProvider for ServerConfig {
    fn tracing_level(&self) -> &config_lib::tracing_level::TracingLevel {
        &self.tracing_level.0
    }
}
impl config_lib::domain_types::EnableApiGitCommitCheckProvider for ServerConfig {
    fn enable_api_git_commit_check(&self) -> &bool {
        &self.enable_api_git_commit_check.0
    }
}
impl config_lib::admin_cookie_secure::AdminCookieSecureProvider for ServerConfig {
    fn admin_cookie_secure(&self) -> &bool {
        &self.admin_cookie_secure
    }
}
impl config_lib::admin_jwt_secret::AdminJwtSecretProvider for ServerConfig {
    fn admin_jwt_secret(
        &self,
    ) -> &bounded_types::bounded_vec::BoundedVec<
        config_lib::secrecy_secret_box_string::SecrecySecretBoxString,
        1,
        8,
    > {
        self.admin_jwt_secret.as_ref()
    }
}
impl config_lib::admin_token_audience::AdminTokenAudienceProvider for ServerConfig {
    fn admin_token_audience(&self) -> &String {
        self.admin_token_audience.as_ref()
    }
}
impl config_lib::admin_token_issuer::AdminTokenIssuerProvider for ServerConfig {
    fn admin_token_issuer(&self) -> &String {
        self.admin_token_issuer.as_ref()
    }
}
