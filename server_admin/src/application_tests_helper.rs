fn admin_app_test_env<Value>(value: &str) -> Value
where
    Value: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    Value::Error: std::fmt::Debug,
{
    Value::try_from_std_env_var_ok(
        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value.to_owned())
            .expect("82c951d4 env invariant must hold"),
    )
    .expect("135a22e8 env invariant must hold")
}

pub(crate) fn auth_state(
    pool: sqlx::PgPool,
    allowed_origin: &str,
) -> Result<
    crate::admin_auth_svc_state::AdminAuthSvcState,
    crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError,
> {
    crate::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool),
        &admin_app_test_env(constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES),
        &admin_app_test_env(constants_str::VALUE_900),
        &admin_app_test_env(constants_str::VALUE_3600),
        &admin_app_test_env(constants_str::VALUE_20),
        &admin_app_test_env(constants_str::VALUE_2),
        &admin_app_test_env(constants_str::VALUE_10),
        &admin_app_test_env(constants_str::VALUE_1),
        &admin_app_test_env(constants_str::FALSE),
        &admin_app_test_env(constants_str::INTEGRATION_TEST),
        &admin_app_test_env(constants_str::INTEGRATION_TEST_ADMIN),
        &config_lib::domain_types::CorsAllowOrigin(allowed_origin.to_owned()),
    )
}
