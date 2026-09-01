fn make_git_info() -> git_info::project_git_info::ProjectGitInfo<'static> {
    git_info::project_git_info::ProjectGitInfo::from(
        git_info::git_commit_id_ref::GitCommitIdRef::from(constants_str::TEST_VALUES_COMMIT),
    )
}
fn app_state_test_env<T>(value: &str) -> T
where
    T: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(
        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value.to_owned())
            .expect(constants_str::DIAGNOSTIC_53A63100),
    )
    .expect(constants_str::DIAGNOSTIC_3879E38D)
}
fn make_structure(
    project_git_info: git_info::project_git_info::ProjectGitInfo<'_>,
) -> crate::server_app_state::ServerAppState<'_> {
    crate::server_app_state::ServerAppState::new(
        server_runtime_core::resource_budget::ResourceBudget::new(
            server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(128usize)
                .expect(constants_str::DIAGNOSTIC_837F89A0),
        ),
        server_config::server_config::ServerConfig::new(
            app_state_test_env(constants_str::ASTERISK),
            app_state_test_env(constants_str::TEST_CONTENT_SECURITY_POLICY),
            app_state_test_env(constants_str::POSTGRES_DB),
            app_state_test_env(constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES),
            app_state_test_env(constants_str::TEST_AUDIENCE),
            app_state_test_env(constants_str::TEST_ISSUER),
            app_state_test_env(constants_str::VALUE_127_0_0_1_32_PATH_1_128),
            app_state_test_env(constants_str::VALUE_900),
            app_state_test_env(constants_str::VALUE_4),
            app_state_test_env(constants_str::VALUE_10),
            app_state_test_env(constants_str::VALUE_2592000),
            app_state_test_env(constants_str::VALUE_20),
            app_state_test_env(constants_str::VALUE_10),
            app_state_test_env(constants_str::TEST_VALUE_30),
            app_state_test_env(constants_str::TEST_VALUE_30),
            app_state_test_env(constants_str::TEST_VALUE_30),
            app_state_test_env(constants_str::TEST_VALUE_30),
            config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes::try_from(
                16_384,
            )
            .expect(constants_str::DIAGNOSTIC_D81F6A42),
            app_state_test_env(constants_str::VALUE_127_0_0_1_3000),
            config_lib::pg_pool_max_connections::PgPoolMaxConnections::try_from(7)
                .expect(constants_str::DIAGNOSTIC_F20C4A91),
            app_state_test_env(constants_str::VALUE_0),
            config_lib::chrono_timezone::ChronoTimezone::try_from(
                chrono::FixedOffset::east_opt(3i32 * 3_600i32)
                    .expect(constants_str::DIAGNOSTIC_A95D3C17),
            )
            .expect(constants_str::DIAGNOSTIC_E8714250),
            app_state_test_env(constants_str::GITHUB_ALT),
            app_state_test_env(constants_str::CONFIG_TRACING_INFO),
            config_lib::tracing_format::TracingFormat::Text,
            app_state_test_env(constants_str::TRUE),
            app_state_test_env(constants_str::FALSE),
            app_state_test_env(constants_str::TRUE),
            app_state_test_env(constants_str::TRUE),
            config_lib::production_mode::ProductionMode::from(false),
            config_lib::svc_mode::SvcMode::Serve,
        ),
        server_runtime_core::resource_budget::ResourceBudget::new(
            server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(
                constants_usize::VALUE_1_048_576,
            )
            .expect(constants_str::DIAGNOSTIC_926CE310),
        ),
        app_state::sqlx_pg_pool::SqlxPgPool::from(
            sqlx::PgPool::connect_lazy(constants_str::POSTGRES_USR_PWD_LOCALHOST_5432_DB)
                .expect(constants_str::DIAGNOSTIC_4BD3F0A1),
        ),
        project_git_info,
    )
}
#[tokio::test]
#[cfg_attr(
    miri,
    ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
)]
async fn test_cfg_accessors_forward_to_inner_config() {
    let git_info = make_git_info();
    let structure = make_structure(git_info);
    assert_eq!(
        config_lib::domain_types::SrcPlaceTypeProvider::src_place_type(&structure),
        &config_lib::src_place_type::SrcPlaceType::Github
    );
    assert_eq!(
        config_lib::chrono_timezone::ChronoTimezoneProvider::chrono_timezone(&structure)
            .local_minus_utc(),
        3i32 * 3_600i32
    );
    assert_eq!(
            config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(
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
async fn test_sqlx_pg_pool_returns_same_pool_ref() {
    let git_info = make_git_info();
    let structure = make_structure(git_info);
    let lhs = std::ptr::from_ref(
        app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider::sqlx_pg_pool(&structure).as_ref(),
    );
    let rhs = std::ptr::from_ref(
        app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider::sqlx_pg_pool(&structure).as_ref(),
    );
    assert_eq!(lhs, rhs);
}
#[tokio::test]
#[cfg_attr(
    miri,
    ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
)]
async fn test_as_ref_and_git_commit_link_are_consistent() {
    let git_info = make_git_info();
    let structure = make_structure(git_info);
    assert_eq!(structure.as_ref(), constants_str::TEST_VALUES_COMMIT);
    assert_eq!(
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(
            &structure
        ),
        git_info::build_git_commit_link::build_git_commit_link(constants_str::TEST_VALUES_COMMIT)
    );
}
