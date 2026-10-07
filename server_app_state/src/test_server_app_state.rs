fn make_git_info() -> git_info::project_git_info::ProjectGitInfo<'static> {
    git_info::project_git_info::ProjectGitInfo::from(
        git_info::git_commit_id_ref::GitCommitIdRef::from(constants_str::TEST_VALUES_COMMIT),
    )
}
fn app_state_test_env<T>(str: &str) -> T
where
    T: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(
        config_lib::std_env_var_ok::StdEnvVarOk::try_from(str.to_owned())
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
            config_lib::service_mode::ServiceMode::Serve,
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
    let bulk_item_budget = server_runtime_core::bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider::bulk_item_resource_budget(&structure);
    let idempotency_response_budget = server_runtime_core::idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider::idempotency_response_resource_budget(&structure);
    let bulk_item_reservation = bulk_item_budget
        .reserve(server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(128usize))
        .unwrap_or_else(|error| std::panic::panic_any(error));
    assert_eq!(*server_runtime_core::bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider::bulk_item_resource_budget(&structure).reserved(), 128usize);
    assert_eq!(*idempotency_response_budget.reserved(), 0usize);
    let idempotency_response_reservation = idempotency_response_budget
        .reserve(
            server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(
                constants_usize::VALUE_1_048_576,
            ),
        )
        .unwrap_or_else(|error| std::panic::panic_any(error));
    assert_eq!(*server_runtime_core::idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider::idempotency_response_resource_budget(&structure).reserved(), constants_usize::VALUE_1_048_576);
    [bulk_item_budget, idempotency_response_budget].into_iter().fold((), |(), resource_budget| {
        assert!(resource_budget.reserve(server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(1usize)).is_err_and(|error| error == server_runtime_core::resource_budget_reserve_error::ResourceBudgetReserveError::Exhausted));
    });
    drop(bulk_item_reservation);
    assert_eq!(*bulk_item_budget.reserved(), 0usize);
    assert_eq!(
        *idempotency_response_budget.reserved(),
        constants_usize::VALUE_1_048_576
    );
    drop(idempotency_response_reservation);
    assert_eq!(*idempotency_response_budget.reserved(), 0usize);
    assert!(
        !config_lib::admin_cookie_secure::AdminCookieSecureProvider::admin_cookie_secure(
            &structure
        )
    );
    assert_eq!(
        config_lib::admin_token_audience::AdminTokenAudienceProvider::admin_token_audience(
            &structure
        )
        .as_str(),
        constants_str::TEST_AUDIENCE
    );
    assert_eq!(
        config_lib::admin_token_issuer::AdminTokenIssuerProvider::admin_token_issuer(&structure)
            .as_str(),
        constants_str::TEST_ISSUER
    );
    let secrets =
        config_lib::admin_jwt_secret::AdminJwtSecretProvider::admin_jwt_secret(&structure);
    assert_eq!(secrets.len().get(), 1usize);
    assert!(std::ptr::eq(
        secrets,
        config_lib::admin_jwt_secret::AdminJwtSecretProvider::admin_jwt_secret(&structure)
    ));
    assert!(
        secrets
            .iter()
            .all(|secret| format!("{secret:?}") == constants_str::REDACTED_ALT_3)
    );
    assert_eq!(
        config_lib::domain_types::SourcePlaceTypeProvider::source_place_type(&structure),
        &config_lib::source_place_type::SourcePlaceType::Github
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
