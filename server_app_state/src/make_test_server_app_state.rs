#[cfg(feature = "test-utils")]
#[must_use]
pub fn make_test_server_app_state() -> crate::server_app_state::ServerAppState<'static> {
    crate::server_app_state::ServerAppState::new(
        server_runtime_core::resource_budget::ResourceBudget::new(
            server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(8usize)
                .expect("86d3d452 make_test_server_app_state invariant must hold"),
        ),
        server_config::server_config::ServerConfig::new(
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::ASTERISK.to_owned(),
                )
                .expect("512933af test CORS origin fixture must be valid"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_CONTENT_SECURITY_POLICY.to_owned(),
                )
                .expect("957dc3b8 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUES_UNREACHABLE_DATABASE_URL.to_owned(),
                )
                .expect("3e33c100 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
                )
                .expect("f29cc79a make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_AUDIENCE.to_owned(),
                )
                .expect("5b218444 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_ISSUER.to_owned(),
                )
                .expect("8357484d make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_127_0_0_1_32_PATH_1_128.to_owned(),
                )
                .expect("45076555 trusted-proxy fixture must be valid"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_900.to_owned(),
                )
                .expect("4e1b2430 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_1.to_owned(),
                )
                .expect("763e1bd9 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_10.to_owned(),
                )
                .expect("fb8d620e make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_3600.to_owned(),
                )
                .expect("467a6513 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_20.to_owned(),
                )
                .expect("b26f4a08 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_10.to_owned(),
                )
                .expect("53224f39 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("48634ca9 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("4d68545f make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("8b271546 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::TEST_VALUE_30.to_owned(),
                )
                .expect("1e6a4c92 make_test_server_app_state invariant must hold"),
            ),
            config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes::try_from(
                1_024usize,
            )
            .expect("d7a590e3 make_test_server_app_state invariant must hold"),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_127_0_0_1_3000.to_owned(),
                )
                .expect("9cba6537 make_test_server_app_state invariant must hold"),
            ),
            config_lib::pg_pool_max_connections::PgPoolMaxConnections::try_from(1u32)
                .expect("58530f0e make_test_server_app_state invariant must hold"),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::VALUE_0.to_owned(),
                )
                .expect("d816fc9a make_test_server_app_state invariant must hold"),
            ),
            config_lib::chrono_timezone::ChronoTimezone::try_from(
                chrono::FixedOffset::east_opt(10_800i32)
                    .expect("695a2c2a make_test_server_app_state invariant must hold"),
            )
            .expect("e3e42aa5 make_test_server_app_state invariant must hold"),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::GITHUB_ALT.to_owned(),
                )
                .expect("a67f33d1 source-place fixture must be valid"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(
                    constants_str::CONFIG_TRACING_INFO.to_owned(),
                )
                .expect("10d6fa4b tracing-level fixture must be valid"),
            ),
            config_lib::tracing_format::TracingFormat::Text,
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(constants_str::FALSE.to_owned())
                    .expect("920ef9aa git-check fixture must be valid"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(constants_str::FALSE.to_owned())
                    .expect("dbe97ef3 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(constants_str::TRUE.to_owned())
                    .expect("818b46e8 make_test_server_app_state invariant must hold"),
            ),
            crate::test_env::test_env(
                config_lib::std_env_var_ok::StdEnvVarOk::try_from(constants_str::TRUE.to_owned())
                    .expect("7c36108e make_test_server_app_state invariant must hold"),
            ),
            config_lib::production_mode::ProductionMode::from(false),
            config_lib::svc_mode::SvcMode::Serve,
        ),
        server_runtime_core::resource_budget::ResourceBudget::new(
            server_runtime_core::resource_budget_maximum::ResourceBudgetMaximum::try_from(
                4_096usize,
            )
            .expect("799dc227 make_test_server_app_state invariant must hold"),
        ),
        app_state::sqlx_pg_pool::SqlxPgPool::from(
            sqlx::PgPool::connect_lazy(constants_str::TEST_VALUES_UNREACHABLE_DATABASE_URL)
                .expect("d53d8ff0 make_test_server_app_state invariant must hold"),
        ),
        git_info::project_git_info_value::project_git_info_value(),
    )
}
