#[cfg(test)]
mod tests {
    #[test]
    fn test_pool_limits_and_timeouts_reject_zero() {
        let max =
            <crate::pg_pool_max_connections::PgPoolMaxConnections as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1))
                    .expect(constants_str::DIAGNOSTIC_6F71A4B9),
            )
            .expect(constants_str::DIAGNOSTIC_C8EF416D);
        assert_eq!(*max, 1u32);
        let timeout =
            <crate::request_timeout_seconds::RequestTimeoutSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0))
                    .expect(constants_str::DIAGNOSTIC_F02D58B1),
            );
        assert!(matches!(
            timeout,
            Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Zero)
        ));
    }
}
