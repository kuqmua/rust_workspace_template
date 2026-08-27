#[path = "config.rs"]
mod config;
#[path = "production_config_error.rs"]
mod production_config_error;

pub use config::{Config, ConfigTryFromEnvError};
pub use production_config_error::ProductionConfigError;

#[cfg(test)]
mod tests {
    fn env<T>(value: &str) -> T
    where
        T: config_lib::domain_types::TryFromStdEnvVarOk,
        T::Error: std::fmt::Debug,
    {
        T::try_from_std_env_var_ok(
            config_lib::domain_types::StdEnvVarOk::try_from(value.to_owned())
                .expect("aa12cd88 env invariant must hold"),
        )
        .expect("741e5201 env invariant must hold")
    }
    #[test]
    fn generated_accessors_return_expected_refs_and_values() {
        let mut cfg =
            super::Config {
                cors_allow_origin: config_lib::domain_types::CorsAllowOrigin(constants_str::ASTERISK.to_owned()),
                content_security_policy: env(constants_str::TEST_CONTENT_SECURITY_POLICY),
                database_url: env(constants_str::POSTGRES_DB),
                admin_jwt_secret: env(constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES),
                admin_token_audience: env(constants_str::TEST_AUDIENCE),
                admin_token_issuer: env(constants_str::TEST_ISSUER),
                admin_access_token_ttl_seconds: env(constants_str::VALUE_900),
                admin_login_failure_limit: env(constants_str::VALUE_10),
                admin_password_hash_concurrency: env(constants_str::VALUE_4),
                admin_refresh_token_ttl_seconds: env(constants_str::VALUE_2592000),
                admin_session_limit: env(constants_str::VALUE_20),
                admin_sign_in_rate_limit: env(constants_str::VALUE_10),
                admin_swagger_enabled: env(constants_str::TRUE),
                http_gzip_enabled: env(constants_str::TRUE),
                maximum_size_of_http_body_in_bytes:
                    config_lib::domain_types::MaximumSizeOfHttpBodyInBytes::try_from(16_384).expect("0d9e4b7a generated_getters_return_expected_refs_and_values invariant must hold"),
                service_socket_address: config_lib::domain_types::ServiceSocketAddress(
                    constants_str::VALUE_127_0_0_1_8080
                        .parse()
                        .expect("e7a3d5c1 generated_getters_return_expected_refs_and_values invariant must hold"),
                ),
                pg_pool_max_connections: config_lib::domain_types::PgPoolMaxConnections::try_from(8)
                    .expect("39a84c10 generated_getters_return_expected_refs_and_values invariant must hold"),
                pg_pool_min_connections: env(constants_str::VALUE_0),
                pg_pool_acquire_timeout_seconds: env(constants_str::TEST_VALUE_30),
                pg_pool_idle_timeout_seconds: env(constants_str::TEST_VALUE_30),
                pg_pool_max_lifetime_seconds: env(constants_str::TEST_VALUE_30),
                request_timeout_seconds: env(constants_str::TEST_VALUE_30),
                timezone: config_lib::domain_types::ChronoTimezone::try_from(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("93cbf4a2 generated_getters_return_expected_refs_and_values invariant must hold"),
                )
                .expect("50e91ec9 generated_getters_return_expected_refs_and_values invariant must hold"),
                src_place_type: config_lib::domain_types::SrcPlaceType(config_lib::domain_types::types::SrcPlaceType::Github),
                tracing_level: config_lib::domain_types::TracingLevel(config_lib::domain_types::types::TracingLevel::Info),
                tracing_format: config_lib::domain_types::types::TracingFormat::Text,
                trusted_proxy_ranges_text: config_lib::domain_types::TrustedProxyRangesText(
                    constants_str::VALUE_127_0_0_1_32_PATH_1_128.to_owned(),
                ),
                enable_api_git_commit_check: config_lib::domain_types::EnableApiGitCommitCheck(true),
                admin_cookie_secure: env(constants_str::FALSE),
                production_mode: env(constants_str::FALSE),
                svc_mode: config_lib::domain_types::types::SvcMode::Serve,
            };
        assert_eq!(
            config_lib::domain_types::CorsAllowOriginProvider::cors_allow_origin(&cfg),
            "*"
        );
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(
                config_lib::domain_types::DatabaseUrlProvider::database_url(&cfg)
            )
            .as_ref(),
            "postgres://db"
        );
        assert_eq!(
            config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(
                &cfg
            ),
            &16_384
        );
        assert_eq!(
            config_lib::domain_types::ServiceSocketAddressProvider::service_socket_address(&cfg)
                .port(),
            8080
        );
        assert_eq!(
            config_lib::domain_types::PgPoolMaxConnectionsProvider::pg_pool_max_connections(&cfg),
            &8
        );
        assert_eq!(
            config_lib::domain_types::ChronoTimezoneProvider::chrono_timezone(&cfg)
                .local_minus_utc(),
            3i32 * 3_600i32
        );
        assert_eq!(
            config_lib::domain_types::SrcPlaceTypeProvider::src_place_type(&cfg),
            &config_lib::domain_types::types::SrcPlaceType::Github
        );
        assert_eq!(
            config_lib::domain_types::TracingLevelProvider::tracing_level(&cfg),
            &config_lib::domain_types::types::TracingLevel::Info
        );
        assert!(
            config_lib::domain_types::EnableApiGitCommitCheckProvider::enable_api_git_commit_check(
                &cfg
            )
        );
        assert_eq!(cfg.validate_for_startup(), Ok(()));
        cfg.production_mode = config_lib::domain_types::ProductionMode::from(true);
        assert_eq!(
            cfg.validate_for_startup(),
            Err(super::ProductionConfigError::AdminCookieInsecure)
        );
        cfg.admin_cookie_secure = config_lib::domain_types::AdminCookieSecure::from(true);
        assert_eq!(
            cfg.validate_for_startup(),
            Err(super::ProductionConfigError::AdminSwaggerEnabled)
        );
        cfg.admin_swagger_enabled = config_lib::domain_types::AdminSwaggerEnabled::from(false);
        assert_eq!(
            cfg.validate_for_startup(),
            Err(super::ProductionConfigError::CorsOriginInsecure)
        );
        cfg.cors_allow_origin =
            config_lib::domain_types::CorsAllowOrigin(constants_str::HTTPS_EXAMPLE_COM.to_owned());
        cfg.admin_jwt_secret = env(constants_str::ADMIN_DEVELOPMENT_JWT_SECRET);
        assert_eq!(
            cfg.validate_for_startup(),
            Err(super::ProductionConfigError::DevelopmentJwtSecret)
        );
        cfg.admin_jwt_secret = env(constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES);
        assert_eq!(cfg.validate_for_startup(), Ok(()));
    }
}
