#[cfg(feature = "test-utils")]
use crate::domain_types::{ServerAppState, test_env};

#[cfg(feature = "test-utils")]
#[must_use]
pub fn make_test_server_app_state() -> ServerAppState<'static> {
    ServerAppState {
        bulk_item_budget: server_runtime_core::ResourceBudget::new(
            server_runtime_core::ResourceBudgetMaximum::try_from(8usize)
                .expect("86d3d452 make_test_server_app_state invariant must hold"),
        ),
        config: server_config::domain_types::Config {
            svc_mode: config_lib::domain_types::types::SvcMode::Serve,
            cors_allow_origin: config_lib::domain_types::CorsAllowOrigin(
                constants_str::ASTERISK.to_owned(),
            ),
            content_security_policy: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_CONTENT_SECURITY_POLICY.to_owned(),
                )
                .expect("957dc3b8 make_test_server_app_state invariant must hold"),
            ),
            database_url: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUES_UNREACHABLE_DATABASE_URL.to_owned(),
                )
                .expect("3e33c100 make_test_server_app_state invariant must hold"),
            ),
            admin_jwt_secret: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
                )
                .expect("f29cc79a make_test_server_app_state invariant must hold"),
            ),
            admin_token_audience: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_AUDIENCE.to_owned(),
                )
                .expect("5b218444 make_test_server_app_state invariant must hold"),
            ),
            admin_token_issuer: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_ISSUER.to_owned(),
                )
                .expect("8357484d make_test_server_app_state invariant must hold"),
            ),
            admin_access_token_ttl_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::VALUE_900.to_owned(),
                )
                .expect("4e1b2430 make_test_server_app_state invariant must hold"),
            ),
            admin_password_hash_concurrency: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_1.to_owned())
                    .expect("763e1bd9 make_test_server_app_state invariant must hold"),
            ),
            admin_login_failure_limit: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_10.to_owned())
                    .expect("fb8d620e make_test_server_app_state invariant must hold"),
            ),
            admin_refresh_token_ttl_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::VALUE_3600.to_owned(),
                )
                .expect("467a6513 make_test_server_app_state invariant must hold"),
            ),
            admin_session_limit: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_20.to_owned())
                    .expect("b26f4a08 make_test_server_app_state invariant must hold"),
            ),
            admin_sign_in_rate_limit: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_10.to_owned())
                    .expect("53224f39 make_test_server_app_state invariant must hold"),
            ),
            admin_swagger_enabled: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::TRUE.to_owned())
                    .expect("818b46e8 make_test_server_app_state invariant must hold"),
            ),
            http_gzip_enabled: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::TRUE.to_owned())
                    .expect("7c36108e make_test_server_app_state invariant must hold"),
            ),
            production_mode: config_lib::domain_types::ProductionMode::from(false),
            maximum_size_of_http_body_in_bytes:
                config_lib::domain_types::MaximumSizeOfHttpBodyInBytes::try_from(1_024usize)
                    .expect("d7a590e3 make_test_server_app_state invariant must hold"),
            service_socket_address: config_lib::domain_types::ServiceSocketAddress(
                constants_str::VALUE_127_0_0_1_3000
                    .parse()
                    .expect("9cba6537 make_test_server_app_state invariant must hold"),
            ),
            pg_pool_max_connections: config_lib::domain_types::PgPoolMaxConnections::try_from(1u32)
                .expect("58530f0e make_test_server_app_state invariant must hold"),
            pg_pool_min_connections: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_0.to_owned())
                    .expect("d816fc9a make_test_server_app_state invariant must hold"),
            ),
            pg_pool_acquire_timeout_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("48634ca9 make_test_server_app_state invariant must hold"),
            ),
            pg_pool_idle_timeout_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("4d68545f make_test_server_app_state invariant must hold"),
            ),
            pg_pool_max_lifetime_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("8b271546 make_test_server_app_state invariant must hold"),
            ),
            request_timeout_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("1e6a4c92 make_test_server_app_state invariant must hold"),
            ),
            timezone: config_lib::domain_types::ChronoTimezone::try_from(
                chrono::FixedOffset::east_opt(10_800i32)
                    .expect("695a2c2a make_test_server_app_state invariant must hold"),
            )
            .expect("e3e42aa5 make_test_server_app_state invariant must hold"),
            src_place_type: config_lib::domain_types::SrcPlaceType(
                config_lib::domain_types::types::SrcPlaceType::Github,
            ),
            tracing_level: config_lib::domain_types::TracingLevel(
                config_lib::domain_types::types::TracingLevel::Info,
            ),
            tracing_format: config_lib::domain_types::types::TracingFormat::Text,
            trusted_proxy_ranges_text: config_lib::domain_types::TrustedProxyRangesText(
                constants_str::VALUE_127_0_0_1_32_PATH_1_128.to_owned(),
            ),
            enable_api_git_commit_check: config_lib::domain_types::EnableApiGitCommitCheck(false),
            admin_cookie_secure: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::FALSE.to_owned())
                    .expect("dbe97ef3 make_test_server_app_state invariant must hold"),
            ),
        },
        idempotency_response_budget: server_runtime_core::ResourceBudget::new(
            server_runtime_core::ResourceBudgetMaximum::try_from(4_096usize)
                .expect("799dc227 make_test_server_app_state invariant must hold"),
        ),
        pg_pool: app_state::SqlxPgPool::from(
            sqlx::PgPool::connect_lazy(constants_str::TEST_VALUES_UNREACHABLE_DATABASE_URL)
                .expect("d53d8ff0 make_test_server_app_state invariant must hold"),
        ),
        project_git_info: git_info::project_git_info_value(),
    }
}
