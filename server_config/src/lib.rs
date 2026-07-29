#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, try_from_env::TryFromEnv, optml::Optml)]
#[config(env_example)]
pub struct Config {
    #[config(example = "http://127.0.0.1:8080")]
    pub cors_allow_origin: config_lib::CorsAllowOrigin,
    #[config(
        example = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; object-src 'none'; frame-ancestors 'none'"
    )]
    pub content_security_policy: config_lib::ContentSecurityPolicy,
    #[config(secret)]
    #[config(example = "postgres://postgres:change-me@127.0.0.1:5432/rust_workspace_template")]
    pub database_url: config_lib::DatabaseUrl,
    #[config(secret)]
    #[config(example = "change-me-development-secret")]
    pub admin_jwt_secret: config_lib::AdminJwtSecret,
    #[config(example = "rust-workspace-template")]
    pub admin_token_audience: config_lib::AdminTokenAudience,
    #[config(example = "rust-workspace-template")]
    pub admin_token_issuer: config_lib::AdminTokenIssuer,
    #[config(example = "127.0.0.1/32,::1/128")]
    pub trusted_proxy_ranges_text: config_lib::TrustedProxyRangesText,
    #[config(example = "900")]
    pub admin_access_token_ttl_seconds: config_lib::AdminAccessTokenTtlSeconds,
    #[config(example = "2")]
    pub admin_password_hash_concurrency: config_lib::AdminPasswordHashConcurrency,
    #[config(example = "604800")]
    pub admin_refresh_token_ttl_seconds: config_lib::AdminRefreshTokenTtlSeconds,
    #[config(example = "8")]
    pub admin_session_limit: config_lib::AdminSessionLimit,
    #[config(example = "10")]
    pub admin_sign_in_rate_limit: config_lib::AdminSignInRateLimit,
    #[config(example = "10")]
    pub pg_pool_acquire_timeout_seconds: config_lib::PgPoolAcquireTimeoutSeconds,
    #[config(example = "600")]
    pub pg_pool_idle_timeout_seconds: config_lib::PgPoolIdleTimeoutSeconds,
    #[config(example = "1800")]
    pub pg_pool_max_lifetime_seconds: config_lib::PgPoolMaxLifetimeSeconds,
    #[config(example = "30")]
    pub request_timeout_seconds: config_lib::RequestTimeoutSeconds,
    #[config(example = "1048576")]
    pub maximum_size_of_http_body_in_bytes: config_lib::MaximumSizeOfHttpBodyInBytes,
    #[config(example = "127.0.0.1:8080")]
    pub service_socket_address: config_lib::ServiceSocketAddress,
    #[config(example = "10")]
    pub pg_pool_max_connections: config_lib::PgPoolMaxConnections,
    #[config(example = "1")]
    pub pg_pool_min_connections: config_lib::PgPoolMinConnections,
    #[config(example = "10800")]
    pub timezone: config_lib::ChronoTimezone,
    #[config(example = "src")]
    pub src_place_type: config_lib::SrcPlaceType,
    #[config(example = "info")]
    pub tracing_level: config_lib::TracingLevel,
    #[config(example = "text")]
    pub tracing_format: config_lib::types::TracingFormat,
    #[config(example = "true")]
    pub enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck,
    #[config(example = "false")]
    pub admin_cookie_secure: config_lib::AdminCookieSecure,
    #[config(example = "true")]
    pub admin_swagger_enabled: config_lib::AdminSwaggerEnabled,
    #[config(example = "true")]
    pub http_gzip_enabled: config_lib::HttpGzipEnabled,
}
impl config_lib::GetCorsAllowOrigin for Config {
    fn get_cors_allow_origin(&self) -> &String {
        &self.cors_allow_origin.0
    }
}
impl config_lib::GetDatabaseUrl for Config {
    fn get_database_url(&self) -> &secrecy::SecretBox<config_lib::StdConfigSecretString> {
        &self.database_url.0
    }
}
impl config_lib::GetMaximumSizeOfHttpBodyInBytes for Config {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &usize {
        &self.maximum_size_of_http_body_in_bytes
    }
}
impl config_lib::GetServiceSocketAddress for Config {
    fn get_service_socket_address(&self) -> &std::net::SocketAddr {
        &self.service_socket_address.0
    }
}
impl config_lib::GetPgPoolMaxConnections for Config {
    fn get_pg_pool_max_connections(&self) -> &u32 {
        &self.pg_pool_max_connections
    }
}
impl config_lib::GetChronoTimezone for Config {
    fn get_chrono_timezone(&self) -> &chrono::FixedOffset {
        &self.timezone
    }
}
impl config_lib::GetSrcPlaceType for Config {
    fn get_src_place_type(&self) -> &config_lib::types::SrcPlaceType {
        &self.src_place_type.0
    }
}
impl config_lib::GetTracingLevel for Config {
    fn get_tracing_level(&self) -> &config_lib::types::TracingLevel {
        &self.tracing_level.0
    }
}
impl config_lib::GetEnableApiGitCommitCheck for Config {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        &self.enable_api_git_commit_check.0
    }
}
impl config_lib::GetAdminAccessTokenTtlSeconds for Config {
    fn get_admin_access_token_ttl_seconds(&self) -> &config_lib::StdNonZeroU64 {
        &self.admin_access_token_ttl_seconds
    }
}
impl config_lib::GetAdminCookieSecure for Config {
    fn get_admin_cookie_secure(&self) -> &bool {
        &self.admin_cookie_secure
    }
}
impl config_lib::GetAdminJwtSecret for Config {
    fn get_admin_jwt_secret(
        &self,
    ) -> &bounded_types::BoundedVec<config_lib::SecrecySecretBoxString, 1, 8> {
        self.admin_jwt_secret.as_ref()
    }
}
impl config_lib::GetAdminPasswordHashConcurrency for Config {
    fn get_admin_password_hash_concurrency(&self) -> &config_lib::StdNonZeroUsize {
        &self.admin_password_hash_concurrency
    }
}
impl config_lib::GetAdminRefreshTokenTtlSeconds for Config {
    fn get_admin_refresh_token_ttl_seconds(&self) -> &config_lib::StdNonZeroU64 {
        &self.admin_refresh_token_ttl_seconds
    }
}
impl config_lib::GetAdminTokenAudience for Config {
    fn get_admin_token_audience(&self) -> &String {
        self.admin_token_audience.as_ref()
    }
}
impl config_lib::GetAdminTokenIssuer for Config {
    fn get_admin_token_issuer(&self) -> &String {
        self.admin_token_issuer.as_ref()
    }
}
#[cfg(test)]
mod tests {
    fn env<T>(value: &str) -> T
    where
        T: config_lib::TryFromStdEnvVarOk,
        T::Error: std::fmt::Debug,
    {
        T::try_from_std_env_var_ok(
            config_lib::StdEnvVarOk::try_from(value.to_owned()).expect("aa12cd88"),
        )
        .expect("741e5201")
    }
    #[test]
    fn generated_getters_return_expected_refs_and_values() {
        let cfg =
            super::Config {
                cors_allow_origin: config_lib::CorsAllowOrigin(str_constants::ASTERISK.to_owned()),
                content_security_policy: env(str_constants::TEST_CONTENT_SECURITY_POLICY),
                database_url: env(str_constants::POSTGRES_DB),
                admin_jwt_secret: env(str_constants::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES),
                admin_token_audience: env(str_constants::TEST_AUDIENCE),
                admin_token_issuer: env(str_constants::TEST_ISSUER),
                admin_access_token_ttl_seconds: env(str_constants::VALUE_900),
                admin_password_hash_concurrency: env(str_constants::VALUE_4),
                admin_refresh_token_ttl_seconds: env(str_constants::VALUE_2592000),
                admin_session_limit: env(str_constants::VALUE_20),
                admin_sign_in_rate_limit: env(str_constants::VALUE_10),
                admin_swagger_enabled: env(str_constants::TRUE),
                http_gzip_enabled: env(str_constants::TRUE),
                maximum_size_of_http_body_in_bytes:
                    config_lib::MaximumSizeOfHttpBodyInBytes::try_from(16_384).expect("0d9e4b7a"),
                service_socket_address: config_lib::ServiceSocketAddress(
                    str_constants::VALUE_127_0_0_1_8080
                        .parse()
                        .expect("e7a3d5c1"),
                ),
                pg_pool_max_connections: config_lib::PgPoolMaxConnections::try_from(8)
                    .expect("39a84c10"),
                pg_pool_min_connections: env(str_constants::VALUE_0),
                pg_pool_acquire_timeout_seconds: env(str_constants::TEST_VALUE_30),
                pg_pool_idle_timeout_seconds: env(str_constants::TEST_VALUE_30),
                pg_pool_max_lifetime_seconds: env(str_constants::TEST_VALUE_30),
                request_timeout_seconds: env(str_constants::TEST_VALUE_30),
                timezone: config_lib::ChronoTimezone::try_from(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("93cbf4a2"),
                )
                .expect("50e91ec9"),
                src_place_type: config_lib::SrcPlaceType(config_lib::types::SrcPlaceType::Github),
                tracing_level: config_lib::TracingLevel(config_lib::types::TracingLevel::Info),
                tracing_format: config_lib::types::TracingFormat::Text,
                trusted_proxy_ranges_text: config_lib::TrustedProxyRangesText(
                    str_constants::VALUE_127_0_0_1_32_PATH_1_128.to_owned(),
                ),
                enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck(true),
                admin_cookie_secure: env(str_constants::FALSE),
            };
        assert_eq!(
            config_lib::GetCorsAllowOrigin::get_cors_allow_origin(&cfg),
            "*"
        );
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(config_lib::GetDatabaseUrl::get_database_url(
                &cfg
            ))
            .as_ref(),
            "postgres://db"
        );
        assert_eq!(
            config_lib::GetMaximumSizeOfHttpBodyInBytes::get_maximum_size_of_http_body_in_bytes(
                &cfg
            ),
            &16_384
        );
        assert_eq!(
            config_lib::GetServiceSocketAddress::get_service_socket_address(&cfg).port(),
            8080
        );
        assert_eq!(
            config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(&cfg),
            &8
        );
        assert_eq!(
            config_lib::GetChronoTimezone::get_chrono_timezone(&cfg).local_minus_utc(),
            3i32 * 3_600i32
        );
        assert_eq!(
            config_lib::GetSrcPlaceType::get_src_place_type(&cfg),
            &config_lib::types::SrcPlaceType::Github
        );
        assert_eq!(
            config_lib::GetTracingLevel::get_tracing_level(&cfg),
            &config_lib::types::TracingLevel::Info
        );
        assert!(config_lib::GetEnableApiGitCommitCheck::get_enable_api_git_commit_check(&cfg));
    }
}
