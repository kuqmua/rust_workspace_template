#[derive(Debug, optml::Optml)]
pub struct ServerAppState<'lt> {
    pub config: server_config::Config,
    pub pg_pool: app_state::PgPool,
    pub project_git_info: &'lt git_info::ProjectGitInfo<'lt>,
}
impl ServerAppState<'_> {
    #[allow(clippy::single_call_fn)] // keeps config forwarding in one place for all generated trait impls
    const fn cfg_ref(&self) -> &server_config::Config {
        &self.config
    }
}
impl cmn_routes::CmnRoutesPrms for ServerAppState<'_> {}
impl pg_crud::CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
server_app_state_macros::impl_cfg_getter!(
    app_state::GetEnableApiGitCommitCheck,
    get_enable_api_git_commit_check,
    bool
);
server_app_state_macros::impl_cfg_getter!(
    app_state::GetSrcPlaceType,
    get_src_place_type,
    config_lib::types::SrcPlaceType
);
server_app_state_macros::impl_cfg_getter!(
    app_state::GetTimezone,
    get_timezone,
    chrono::FixedOffset
);
server_app_state_macros::impl_cfg_getter!(
    app_state::GetMaximumSizeOfHttpBodyInBytes,
    get_maximum_size_of_http_body_in_bytes,
    usize
);
impl app_state::GetPgPool for ServerAppState<'_> {
    fn get_pg_pool(&self) -> app_state::PgPoolRef<'_> {
        app_state::PgPoolRef(&self.pg_pool.0)
    }
}
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.commit.0
    }
}
#[cfg(test)]
mod tests {
    const TEST_COMMIT: &str = "abc123";
    #[allow(clippy::single_call_fn)] // shared fixture keeps commit test input consistent across ServerAppState tests
    fn mk_git_info() -> git_info::ProjectGitInfo<'static> {
        git_info::ProjectGitInfo {
            commit: git_info::GitCommitIdRef(TEST_COMMIT),
        }
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
                maximum_size_of_http_body_in_bytes: config_lib::MaximumSizeOfHttpBodyInBytes(
                    16_384,
                ),
                service_socket_address: config_lib::ServiceSocketAddress(
                    "127.0.0.1:3000".parse().expect("73f8bc91"),
                ),
                pg_pool_max_connections: config_lib::PgPoolMaxConnections(7),
                timezone: config_lib::Timezone(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("a95d3c17"),
                ),
                src_place_type: config_lib::SrcPlaceType(config_lib::types::SrcPlaceType::Github),
                tracing_level: config_lib::TracingLevel(config_lib::types::TracingLevel::Info),
                enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck(true),
            },
            pg_pool: app_state::PgPool(
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
            app_state::GetSrcPlaceType::get_src_place_type(&st),
            &config_lib::types::SrcPlaceType::Github
        );
        assert_eq!(
            app_state::GetTimezone::get_timezone(&st).local_minus_utc(),
            3i32 * 3_600i32
        );
        assert_eq!(
            app_state::GetMaximumSizeOfHttpBodyInBytes::get_maximum_size_of_http_body_in_bytes(&st),
            &16_384
        );
        assert!(app_state::GetEnableApiGitCommitCheck::get_enable_api_git_commit_check(&st));
    }
    #[tokio::test]
    async fn get_pg_pool_returns_same_pool_ref() {
        let git_info = mk_git_info();
        let st = mk_st(&git_info);
        let lhs = std::ptr::from_ref(app_state::GetPgPool::get_pg_pool(&st).0);
        let rhs = std::ptr::from_ref(&st.pg_pool.0);
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
