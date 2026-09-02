fn admin_app_test_env<Value>(str: &str) -> Value
where
    Value: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    Value::Error: std::fmt::Debug,
{
    Value::try_from_std_env_var_ok(
        config_lib::std_env_var_ok::StdEnvVarOk::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_82C951D4),
    )
    .expect(constants_str::DIAGNOSTIC_135A22E8)
}

pub(crate) fn auth_state(
    pg_pool: sqlx::PgPool,
    str: &str,
) -> Result<
    crate::admin_auth_svc_state::AdminAuthSvcState,
    crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError,
> {
    let Ok(allowed_origin) = config_lib::domain_types::CorsAllowOrigin::try_from(str.to_owned())
    else {
        return Err(
            crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::AllowedOrigin,
        );
    };
    crate::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pg_pool),
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
        &allowed_origin,
    )
}
