#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ServerAppState<'lt> {
    pub bulk_item_budget: server_runtime_core::domain_types::ResourceBudget,
    pub config: server_config::domain_types::Config,
    pub idempotency_response_budget: server_runtime_core::domain_types::ResourceBudget,
    pub pg_pool: app_state::domain_types::SqlxPgPool,
    pub project_git_info: git_info::domain_types::ProjectGitInfo<'lt>,
}
impl ServerAppState<'_> {
    const fn cfg_ref(&self) -> &server_config::domain_types::Config {
        &self.config
    }
}
impl common_routes::domain_types::CommonRoutesParameters for ServerAppState<'_> {}
impl pg_table::domain_types::CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
impl server_runtime_core::domain_types::BulkItemResourceBudgetProvider for ServerAppState<'_> {
    fn bulk_item_resource_budget(&self) -> &server_runtime_core::domain_types::ResourceBudget {
        &self.bulk_item_budget
    }
}
impl server_runtime_core::domain_types::IdempotencyResponseResourceBudgetProvider
    for ServerAppState<'_>
{
    fn idempotency_response_resource_budget(
        &self,
    ) -> &server_runtime_core::domain_types::ResourceBudget {
        &self.idempotency_response_budget
    }
}
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::EnableApiGitCommitCheckProvider,
    enable_api_git_commit_check,
    bool
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::SrcPlaceTypeProvider,
    src_place_type,
    config_lib::domain_types::types::SrcPlaceType
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::ChronoTimezoneProvider,
    chrono_timezone,
    chrono::FixedOffset
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider,
    maximum_size_of_http_body_in_bytes,
    usize
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminAccessTokenTtlSecondsProvider,
    admin_access_token_ttl_seconds,
    config_lib::domain_types::ConfigNonZeroU64
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminCookieSecureProvider,
    admin_cookie_secure,
    bool
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminJwtSecretProvider,
    admin_jwt_secret,
    bounded_types::domain_types::vector::BoundedVec<config_lib::domain_types::SecrecySecretBoxString, 1, 8>
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminPasswordHashConcurrencyProvider,
    admin_password_hash_concurrency,
    config_lib::domain_types::ConfigNonZeroUsize
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminRefreshTokenTtlSecondsProvider,
    admin_refresh_token_ttl_seconds,
    config_lib::domain_types::ConfigNonZeroU64
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminTokenAudienceProvider,
    admin_token_audience,
    String
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminTokenIssuerProvider,
    admin_token_issuer,
    String
);
impl app_state::domain_types::SqlxPgPoolProvider for ServerAppState<'_> {
    fn sqlx_pg_pool(&self) -> app_state::domain_types::SqlxPgPoolRef<'_> {
        app_state::domain_types::SqlxPgPoolRef::from(self.pg_pool.as_ref())
    }
}
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
#[cfg(feature = "test-utils")]
fn test_env<T>(value: config_lib::domain_types::StdEnvVarOk) -> T
where
    T: config_lib::domain_types::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(value).expect("3f1c7bb7 test_env invariant must hold")
}
#[cfg(feature = "test-utils")]
#[must_use]
pub fn mk_test_server_app_state() -> ServerAppState<'static> {
    ServerAppState {
        bulk_item_budget: server_runtime_core::domain_types::ResourceBudget::new(
            server_runtime_core::domain_types::ResourceBudgetMaximum::try_from(8usize)
                .expect("86d3d452 mk_test_server_app_state invariant must hold"),
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
                .expect("957dc3b8 mk_test_server_app_state invariant must hold"),
            ),
            database_url: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUES_UNREACHABLE_DATABASE_URL.to_owned(),
                )
                .expect("3e33c100 mk_test_server_app_state invariant must hold"),
            ),
            admin_jwt_secret: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
                )
                .expect("f29cc79a mk_test_server_app_state invariant must hold"),
            ),
            admin_token_audience: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_AUDIENCE.to_owned(),
                )
                .expect("5b218444 mk_test_server_app_state invariant must hold"),
            ),
            admin_token_issuer: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_ISSUER.to_owned(),
                )
                .expect("8357484d mk_test_server_app_state invariant must hold"),
            ),
            admin_access_token_ttl_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::VALUE_900.to_owned(),
                )
                .expect("4e1b2430 mk_test_server_app_state invariant must hold"),
            ),
            admin_password_hash_concurrency: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_1.to_owned())
                    .expect("763e1bd9 mk_test_server_app_state invariant must hold"),
            ),
            admin_login_failure_limit: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_10.to_owned())
                    .expect("fb8d620e mk_test_server_app_state invariant must hold"),
            ),
            admin_refresh_token_ttl_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::VALUE_3600.to_owned(),
                )
                .expect("467a6513 mk_test_server_app_state invariant must hold"),
            ),
            admin_session_limit: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_20.to_owned())
                    .expect("b26f4a08 mk_test_server_app_state invariant must hold"),
            ),
            admin_sign_in_rate_limit: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_10.to_owned())
                    .expect("53224f39 mk_test_server_app_state invariant must hold"),
            ),
            admin_swagger_enabled: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::TRUE.to_owned())
                    .expect("818b46e8 mk_test_server_app_state invariant must hold"),
            ),
            http_gzip_enabled: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::TRUE.to_owned())
                    .expect("7c36108e mk_test_server_app_state invariant must hold"),
            ),
            production_mode: config_lib::domain_types::ProductionMode::from(false),
            maximum_size_of_http_body_in_bytes:
                config_lib::domain_types::MaximumSizeOfHttpBodyInBytes::try_from(1_024usize)
                    .expect("d7a590e3 mk_test_server_app_state invariant must hold"),
            service_socket_address: config_lib::domain_types::ServiceSocketAddress(
                constants_str::VALUE_127_0_0_1_3000
                    .parse()
                    .expect("9cba6537 mk_test_server_app_state invariant must hold"),
            ),
            pg_pool_max_connections: config_lib::domain_types::PgPoolMaxConnections::try_from(1u32)
                .expect("58530f0e mk_test_server_app_state invariant must hold"),
            pg_pool_min_connections: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(constants_str::VALUE_0.to_owned())
                    .expect("d816fc9a mk_test_server_app_state invariant must hold"),
            ),
            pg_pool_acquire_timeout_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("48634ca9 mk_test_server_app_state invariant must hold"),
            ),
            pg_pool_idle_timeout_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("4d68545f mk_test_server_app_state invariant must hold"),
            ),
            pg_pool_max_lifetime_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("8b271546 mk_test_server_app_state invariant must hold"),
            ),
            request_timeout_seconds: test_env(
                config_lib::domain_types::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("1e6a4c92 mk_test_server_app_state invariant must hold"),
            ),
            timezone: config_lib::domain_types::ChronoTimezone::try_from(
                chrono::FixedOffset::east_opt(10_800i32)
                    .expect("695a2c2a mk_test_server_app_state invariant must hold"),
            )
            .expect("e3e42aa5 mk_test_server_app_state invariant must hold"),
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
                    .expect("dbe97ef3 mk_test_server_app_state invariant must hold"),
            ),
        },
        idempotency_response_budget: server_runtime_core::domain_types::ResourceBudget::new(
            server_runtime_core::domain_types::ResourceBudgetMaximum::try_from(4_096usize)
                .expect("799dc227 mk_test_server_app_state invariant must hold"),
        ),
        pg_pool: app_state::domain_types::SqlxPgPool::from(
            sqlx::PgPool::connect_lazy(constants_str::TEST_VALUES_UNREACHABLE_DATABASE_URL)
                .expect("d53d8ff0 mk_test_server_app_state invariant must hold"),
        ),
        project_git_info: git_info::domain_types::project_git_info(),
    }
}
#[cfg(test)]
mod tests {
    fn mk_git_info() -> git_info::domain_types::ProjectGitInfo<'static> {
        git_info::domain_types::ProjectGitInfo::from(git_info::domain_types::GitCommitIdRef::from(
            constants_str::TEST_VALUES_COMMIT,
        ))
    }
    fn env<T>(value: &str) -> T
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
    fn mk_structure(
        project_git_info: git_info::domain_types::ProjectGitInfo<'_>,
    ) -> super::ServerAppState<'_> {
        super::ServerAppState {
            bulk_item_budget: server_runtime_core::domain_types::ResourceBudget::new(
                server_runtime_core::domain_types::ResourceBudgetMaximum::try_from(128usize)
                    .expect("837f89a0 mk_structure invariant must hold"),
            ),
            config: server_config::domain_types::Config {
                svc_mode: config_lib::domain_types::types::SvcMode::Serve,
                cors_allow_origin: config_lib::domain_types::CorsAllowOrigin(
                    constants_str::ASTERISK.to_owned(),
                ),
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
                production_mode: config_lib::domain_types::ProductionMode::from(false),
                maximum_size_of_http_body_in_bytes:
                    config_lib::domain_types::MaximumSizeOfHttpBodyInBytes::try_from(16_384)
                        .expect("d81f6a42 mk_structure invariant must hold"),
                service_socket_address: config_lib::domain_types::ServiceSocketAddress(
                    constants_str::VALUE_127_0_0_1_3000
                        .parse()
                        .expect("73f8bc91 mk_structure invariant must hold"),
                ),
                pg_pool_max_connections: config_lib::domain_types::PgPoolMaxConnections::try_from(
                    7,
                )
                .expect("f20c4a91 mk_structure invariant must hold"),
                pg_pool_min_connections: env(constants_str::VALUE_0),
                pg_pool_acquire_timeout_seconds: env(constants_str::TEST_VALUE_30),
                pg_pool_idle_timeout_seconds: env(constants_str::TEST_VALUE_30),
                pg_pool_max_lifetime_seconds: env(constants_str::TEST_VALUE_30),
                request_timeout_seconds: env(constants_str::TEST_VALUE_30),
                timezone: config_lib::domain_types::ChronoTimezone::try_from(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32)
                        .expect("a95d3c17 mk_structure invariant must hold"),
                )
                .expect("e8714250 mk_structure invariant must hold"),
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
                admin_cookie_secure: env(constants_str::FALSE),
            },
            pg_pool: app_state::domain_types::SqlxPgPool::from(
                sqlx::PgPool::connect_lazy(constants_str::POSTGRES_USR_PWD_LOCALHOST_5432_DB)
                    .expect("4bd3f0a1 mk_structure invariant must hold"),
            ),
            idempotency_response_budget: server_runtime_core::domain_types::ResourceBudget::new(
                server_runtime_core::domain_types::ResourceBudgetMaximum::try_from(
                    constants_usize::VALUE_1_048_576,
                )
                .expect("926ce310 mk_structure invariant must hold"),
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
        let git_info = mk_git_info();
        let structure = mk_structure(git_info);
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
        let git_info = mk_git_info();
        let structure = mk_structure(git_info);
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
        let git_info = mk_git_info();
        let structure = mk_structure(git_info);
        assert_eq!(structure.as_ref(), constants_str::TEST_VALUES_COMMIT);
        assert_eq!(
            git_info::domain_types::GitCommitLinkProvider::git_commit_link(&structure),
            git_info::domain_types::git_commit_link(constants_str::TEST_VALUES_COMMIT)
        );
    }
}
