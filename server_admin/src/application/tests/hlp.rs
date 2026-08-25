fn env<Value>(value: &str) -> Value
where
    Value: config_lib::domain_types::TryFromStdEnvVarOk,
    Value::Error: std::fmt::Debug,
{
    Value::try_from_std_env_var_ok(
        config_lib::domain_types::StdEnvVarOk::try_from(value.to_owned())
            .expect("82c951d4 env invariant must hold"),
    )
    .expect("135a22e8 env invariant must hold")
}

pub(in crate::domain_types::auth) fn auth_state(
    pool: sqlx::PgPool,
    allowed_origin: &str,
) -> Result<super::super::AdminAuthSvcState, super::super::AdminAuthSvcStateBuildError> {
    super::super::AdminAuthSvcState::try_new(
        app_state::domain_types::SqlxPgPool::from(pool),
        &env(constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES),
        &env(constants_str::VALUE_900),
        &env(constants_str::VALUE_3600),
        &env(constants_str::VALUE_20),
        &env(constants_str::VALUE_2),
        &env(constants_str::VALUE_10),
        &env(constants_str::VALUE_1),
        &env(constants_str::FALSE),
        &env(constants_str::INTEGRATION_TEST),
        &env(constants_str::INTEGRATION_TEST_ADMIN),
        &config_lib::domain_types::CorsAllowOrigin(allowed_origin.to_owned()),
    )
}
