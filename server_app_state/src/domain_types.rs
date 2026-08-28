#[cfg(feature = "test-utils")]
pub use crate::make_test_server_app_state::make_test_server_app_state;
pub use crate::server_app_state::ServerAppState;
#[cfg(feature = "test-utils")]
pub(crate) use crate::test_env::test_env;

#[cfg(test)]
mod tests {
    fn make_git_info() -> git_info::domain_types::ProjectGitInfo<'static> {
        git_info::domain_types::ProjectGitInfo::from(git_info::domain_types::GitCommitIdRef::from(
            constants_str::TEST_VALUES_COMMIT,
        ))
    }
    fn app_state_test_env<T>(value: &str) -> T
    where
        T: config_lib::domain_types::TryFromStdEnvVarOk,
        T::Error: std::fmt::Debug,
    {
        T::try_from_std_env_var_ok(
            config_lib::domain_types::StdEnvVarOk::try_from(value.to_owned())
                .expect("53a63100 env invariant must hold"),
        )
        .expect("3879e38d env invariant must hold")
    }
    fn make_structure(
        project_git_info: git_info::domain_types::ProjectGitInfo<'_>,
    ) -> super::ServerAppState<'_> {
        super::ServerAppState {
            bulk_item_budget: server_runtime_core::domain_types::ResourceBudget::new(
                server_runtime_core::domain_types::ResourceBudgetMaximum::try_from(128usize)
                    .expect("837f89a0 make_structure invariant must hold"),
            ),
            config: server_config::domain_types::Config {
                svc_mode: config_lib::domain_types::types::SvcMode::Serve,
                cors_allow_origin: config_lib::domain_types::CorsAllowOrigin(
                    constants_str::ASTERISK.to_owned(),
                ),
                content_security_policy: app_state_test_env(
                    constants_str::TEST_CONTENT_SECURITY_POLICY,
                ),
                database_url: app_state_test_env(constants_str::POSTGRES_DB),
                admin_jwt_secret: app_state_test_env(
                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES,
                ),
                admin_token_audience: app_state_test_env(constants_str::TEST_AUDIENCE),
                admin_token_issuer: app_state_test_env(constants_str::TEST_ISSUER),
                admin_access_token_ttl_seconds: app_state_test_env(constants_str::VALUE_900),
                admin_login_failure_limit: app_state_test_env(constants_str::VALUE_10),
                admin_password_hash_concurrency: app_state_test_env(constants_str::VALUE_4),
                admin_refresh_token_ttl_seconds: app_state_test_env(constants_str::VALUE_2592000),
                admin_session_limit: app_state_test_env(constants_str::VALUE_20),
                admin_sign_in_rate_limit: app_state_test_env(constants_str::VALUE_10),
                admin_swagger_enabled: app_state_test_env(constants_str::TRUE),
                http_gzip_enabled: app_state_test_env(constants_str::TRUE),
                production_mode: config_lib::domain_types::ProductionMode::from(false),
                maximum_size_of_http_body_in_bytes:
                    config_lib::domain_types::MaximumSizeOfHttpBodyInBytes::try_from(16_384)
                        .expect("d81f6a42 make_structure invariant must hold"),
                service_socket_address: config_lib::domain_types::ServiceSocketAddress(
                    constants_str::VALUE_127_0_0_1_3000
                        .parse()
                        .expect("73f8bc91 make_structure invariant must hold"),
                ),
                pg_pool_max_connections: config_lib::domain_types::PgPoolMaxConnections::try_from(
                    7,
                )
                .expect("f20c4a91 make_structure invariant must hold"),
                pg_pool_min_connections: app_state_test_env(constants_str::VALUE_0),
                pg_pool_acquire_timeout_seconds: app_state_test_env(constants_str::TEST_VALUE_30),
                pg_pool_idle_timeout_seconds: app_state_test_env(constants_str::TEST_VALUE_30),
                pg_pool_max_lifetime_seconds: app_state_test_env(constants_str::TEST_VALUE_30),
                request_timeout_seconds: app_state_test_env(constants_str::TEST_VALUE_30),
                timezone: config_lib::domain_types::ChronoTimezone::try_from(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32)
                        .expect("a95d3c17 make_structure invariant must hold"),
                )
                .expect("e8714250 make_structure invariant must hold"),
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
                enable_api_git_commit_check: config_lib::domain_types::EnableApiGitCommitCheck(
                    true,
                ),
                admin_cookie_secure: app_state_test_env(constants_str::FALSE),
            },
            pg_pool: app_state::domain_types::SqlxPgPool::from(
                sqlx::PgPool::connect_lazy(constants_str::POSTGRES_USR_PWD_LOCALHOST_5432_DB)
                    .expect("4bd3f0a1 make_structure invariant must hold"),
            ),
            idempotency_response_budget: server_runtime_core::domain_types::ResourceBudget::new(
                server_runtime_core::domain_types::ResourceBudgetMaximum::try_from(
                    constants_usize::VALUE_1_048_576,
                )
                .expect("926ce310 make_structure invariant must hold"),
            ),
            project_git_info,
        }
    }
    #[tokio::test]
    #[cfg_attr(
        miri,
        ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
    )]
    async fn cfg_accessors_forward_to_inner_config() {
        let git_info = make_git_info();
        let structure = make_structure(git_info);
        assert_eq!(
            config_lib::domain_types::SrcPlaceTypeProvider::src_place_type(&structure),
            &config_lib::domain_types::types::SrcPlaceType::Github
        );
        assert_eq!(
            config_lib::domain_types::ChronoTimezoneProvider::chrono_timezone(&structure)
                .local_minus_utc(),
            3i32 * 3_600i32
        );
        assert_eq!(
            config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(
                &structure
            ),
            &16_384
        );
        assert!(
            config_lib::domain_types::EnableApiGitCommitCheckProvider::enable_api_git_commit_check(
                &structure
            )
        );
    }
    #[tokio::test]
    #[cfg_attr(
        miri,
        ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
    )]
    async fn sqlx_pg_pool_returns_same_pool_ref() {
        let git_info = make_git_info();
        let structure = make_structure(git_info);
        let lhs = std::ptr::from_ref(
            app_state::domain_types::SqlxPgPoolProvider::sqlx_pg_pool(&structure).as_ref(),
        );
        let rhs = std::ptr::from_ref(structure.pg_pool.as_ref());
        assert_eq!(lhs, rhs);
    }
    #[tokio::test]
    #[cfg_attr(
        miri,
        ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
    )]
    async fn as_ref_and_git_commit_link_are_consistent() {
        let git_info = make_git_info();
        let structure = make_structure(git_info);
        assert_eq!(structure.as_ref(), constants_str::TEST_VALUES_COMMIT);
        assert_eq!(
            git_info::domain_types::GitCommitLinkProvider::build_git_commit_link(&structure),
            git_info::domain_types::build_git_commit_link(constants_str::TEST_VALUES_COMMIT)
        );
    }
}
