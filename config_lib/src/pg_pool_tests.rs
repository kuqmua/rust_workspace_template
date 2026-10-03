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
    #[test]
    fn test_pool_minimum_connections_accepts_zero_and_u32_maximum() {
        [constants_str::VALUE_0.to_owned(), u32::MAX.to_string()]
            .into_iter()
            .fold((), |(), value| {
                assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(value.clone())
                    .is_ok_and(|std_env_var_ok| {
                        <crate::pg_pool_min_connections::PgPoolMinConnections as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok)
                            .is_ok_and(|minimum| (*minimum).to_string() == value)
                    }));
            });
    }

    #[test]
    fn test_pool_minimum_connections_rejects_invalid_and_overflowing_values() {
        [
            constants_str::VALUE_F1234D75.to_owned(),
            (u64::from(u32::MAX) + 1u64).to_string(),
            constants_str::SPACE.to_owned(),
        ]
        .into_iter()
        .fold((), |(), value| {
            assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(value)
                .is_ok_and(|std_env_var_ok| {
                    matches!(
                        <crate::pg_pool_min_connections::PgPoolMinConnections as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok),
                        Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Parse)
                    )
                }));
        });
    }
    #[test]
    fn test_pool_duration_environment_conversions_preserve_positive_values_and_errors() {
        let parse_durations = |std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk| {
            [
                <crate::pg_pool_acquire_timeout_seconds::PgPoolAcquireTimeoutSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok.clone()).map(|seconds| (*seconds).get()),
                <crate::pg_pool_idle_timeout_seconds::PgPoolIdleTimeoutSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok.clone()).map(|seconds| (*seconds).get()),
                <crate::pg_pool_max_lifetime_seconds::PgPoolMaxLifetimeSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok).map(|seconds| (*seconds).get()),
            ]
        };
        [
            (constants_str::VALUE_1.to_owned(), Ok(1u64)),
            (u64::MAX.to_string(), Ok(u64::MAX)),
            (
                constants_str::VALUE_0.to_owned(),
                Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Zero),
            ),
            (
                constants_str::VALUE_F1234D75.to_owned(),
                Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Parse),
            ),
            (
                constants_str::EMPTY.to_owned(),
                Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Parse),
            ),
            (
                (u128::from(u64::MAX) + 1u128).to_string(),
                Err(crate::pg_pool_config_parse_error::PgPoolConfigParseError::Parse),
            ),
        ]
        .into_iter()
        .fold((), |(), (value, expected)| {
            assert!(
                crate::std_env_var_ok::StdEnvVarOk::try_from(value).is_ok_and(|std_env_var_ok| {
                    parse_durations(std_env_var_ok)
                        .into_iter()
                        .all(|actual| actual == expected)
                })
            );
        });
    }
}
