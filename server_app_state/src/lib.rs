#[derive(Debug, optml::Optml)]
pub struct ServerAppState<'lt> {
    pub config: server_config::Config,
    pub pg_pool: app_state::SqlxPgPool,
    pub project_git_info: &'lt git_info::ProjectGitInfo<'lt>,
}
impl ServerAppState<'_> {
    #[allow(clippy::single_call_fn)] // keeps config forwarding in one place for all generated trait impls
    const fn cfg_ref(&self) -> &server_config::Config {
        &self.config
    }
}
impl cmn_routes::CmnRoutesPrms for ServerAppState<'_> {}
impl pg_tbl::CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetEnableApiGitCommitCheck,
    get_enable_api_git_commit_check,
    bool
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetSrcPlaceType,
    get_src_place_type,
    config_lib::types::SrcPlaceType
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetChronoTimezone,
    get_chrono_timezone,
    chrono::FixedOffset
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetMaximumSizeOfHttpBodyInBytes,
    get_maximum_size_of_http_body_in_bytes,
    usize
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminAccessTokenTtlSeconds,
    get_admin_access_token_ttl_seconds,
    config_lib::StdNonZeroU64
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminCookieSecure,
    get_admin_cookie_secure,
    bool
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminJwtSecret,
    get_admin_jwt_secret,
    config_lib::SecrecySecretBoxString
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminPasswordHashConcurrency,
    get_admin_password_hash_concurrency,
    config_lib::StdNonZeroUsize
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminRefreshTokenTtlSeconds,
    get_admin_refresh_token_ttl_seconds,
    config_lib::StdNonZeroU64
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminTokenAudience,
    get_admin_token_audience,
    String
);
server_app_state_macros::impl_cfg_getter!(
    config_lib::GetAdminTokenIssuer,
    get_admin_token_issuer,
    String
);
impl app_state::GetSqlxPgPool for ServerAppState<'_> {
    fn get_sqlx_pg_pool(&self) -> app_state::SqlxPgPoolRef<'_> {
        app_state::SqlxPgPoolRef::from(self.pg_pool.as_ref())
    }
}
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.commit.as_ref()
    }
}
#[cfg(test)]
mod tests {
    const TEST_COMMIT: &str = "abc123";
    #[allow(clippy::single_call_fn)] // shared fixture keeps commit test input consistent across ServerAppState tests
    fn mk_git_info() -> git_info::ProjectGitInfo<'static> {
        git_info::ProjectGitInfo {
            commit: git_info::GitCommitIdRef::from(TEST_COMMIT),
        }
    }
    fn env<T>(value: &str) -> T
    where
        T: config_lib::TryFromStdEnvVarOk,
        T::Error: std::fmt::Debug,
    {
        T::try_from_std_env_var_ok(
            config_lib::StdEnvVarOk::try_from(value.to_owned()).expect("53a63100"),
        )
        .expect("3879e38d")
    }
    fn mk_st<'state_lt>(
        project_git_info: &'state_lt git_info::ProjectGitInfo<'state_lt>,
    ) -> super::ServerAppState<'state_lt> {
        super::ServerAppState {
            config: server_config::Config {
                cors_allow_origin: config_lib::CorsAllowOrigin("*".to_owned()),
                database_url: config_lib::DatabaseUrl(secrecy::SecretBox::new(Box::new(
                    "postgres://db".to_owned(),
                ))),
                admin_jwt_secret: env("test-only-admin-jwt-secret-with-32-bytes"),
                admin_token_audience: env("test-audience"),
                admin_token_issuer: env("test-issuer"),
                admin_access_token_ttl_seconds: env("900"),
                admin_password_hash_concurrency: env("4"),
                admin_refresh_token_ttl_seconds: env("2592000"),
                admin_session_limit: env("20"),
                admin_sign_in_rate_limit: env("10"),
                admin_swagger_enabled: env("true"),
                maximum_size_of_http_body_in_bytes:
                    config_lib::MaximumSizeOfHttpBodyInBytes::try_from(16_384).expect("d81f6a42"),
                service_socket_address: config_lib::ServiceSocketAddress(
                    "127.0.0.1:3000".parse().expect("73f8bc91"),
                ),
                pg_pool_max_connections: config_lib::PgPoolMaxConnections::try_from(7)
                    .expect("f20c4a91"),
                timezone: config_lib::ChronoTimezone::try_from(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("a95d3c17"),
                )
                .expect("e8714250"),
                src_place_type: config_lib::SrcPlaceType(config_lib::types::SrcPlaceType::Github),
                tracing_level: config_lib::TracingLevel(config_lib::types::TracingLevel::Info),
                enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck(true),
                admin_cookie_secure: env("false"),
            },
            pg_pool: app_state::SqlxPgPool::from(
                sqlx::PgPool::connect_lazy("postgres://usr:pwd@localhost:5432/db")
                    .expect("4bd3f0a1"),
            ),
            project_git_info,
        }
    }
    #[tokio::test]
    async fn cfg_getters_forward_to_inner_config() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        assert_eq!(
            config_lib::GetSrcPlaceType::get_src_place_type(&st),
            &config_lib::types::SrcPlaceType::Github
        );
        assert_eq!(
            config_lib::GetChronoTimezone::get_chrono_timezone(&st).local_minus_utc(),
            3i32 * 3_600i32
        );
        assert_eq!(
            config_lib::GetMaximumSizeOfHttpBodyInBytes::get_maximum_size_of_http_body_in_bytes(
                &st
            ),
            &16_384
        );
        assert!(config_lib::GetEnableApiGitCommitCheck::get_enable_api_git_commit_check(&st));
    }
    #[tokio::test]
    async fn get_pg_pool_returns_same_pool_ref() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        let lhs = std::ptr::from_ref(app_state::GetSqlxPgPool::get_sqlx_pg_pool(&st).as_ref());
        let rhs = std::ptr::from_ref(st.pg_pool.as_ref());
        assert_eq!(lhs, rhs);
    }
    #[tokio::test]
    async fn as_ref_and_git_commit_link_are_consistent() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        assert_eq!(st.as_ref(), TEST_COMMIT);
        assert_eq!(
            git_info::GetGitCommitLink::get_git_commit_link(&st),
            git_info::git_commit_link(TEST_COMMIT)
        );
    }
}
