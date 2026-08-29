#[cfg(test)]
mod tests {
    #[test]
    fn pool_limits_and_timeouts_reject_zero() {
        let max =
            <crate::pg_pool_max_connections::PgPoolMaxConnections as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::VALUE_1))
                    .expect("6f71a4b9 pool_limits_and_timeouts_reject_zero invariant must hold"),
            )
            .expect("c8ef416d pool_limits_and_timeouts_reject_zero invariant must hold");
        assert_eq!(max.0, 1u32);
        let timeout =
            <crate::request_timeout_seconds::RequestTimeoutSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::VALUE_0))
                    .expect("f02d58b1 pool_limits_and_timeouts_reject_zero invariant must hold"),
            );
        assert!(matches!(
            timeout,
            Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Zero)
        ));
    }
}
