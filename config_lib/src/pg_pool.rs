use crate::parse_pg_pool_non_zero_seconds::parse_pg_pool_non_zero_seconds;
pub use crate::pg_pool_acquire_timeout_seconds::PgPoolAcquireTimeoutSeconds;
pub use crate::pg_pool_config_parse_error::PgPoolConfigParseError;
pub use crate::pg_pool_idle_timeout_seconds::PgPoolIdleTimeoutSeconds;
pub use crate::pg_pool_max_connections::*;
pub use crate::pg_pool_max_connections_try_from_u32_error::PgPoolMaxConnectionsTryFromU32Error;
pub use crate::pg_pool_max_lifetime_seconds::PgPoolMaxLifetimeSeconds;
pub use crate::pg_pool_min_connections::PgPoolMinConnections;
pub use crate::request_timeout_seconds::RequestTimeoutSeconds;
pub use crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError;

#[cfg(test)]
mod tests {
    #[test]
    fn pool_limits_and_timeouts_reject_zero() {
        let max =
            <super::PgPoolMaxConnections as crate::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1))
                    .expect("6f71a4b9 pool_limits_and_timeouts_reject_zero invariant must hold"),
            )
            .expect("c8ef416d pool_limits_and_timeouts_reject_zero invariant must hold");
        assert_eq!(max.0, 1u32);
        let timeout =
            <super::RequestTimeoutSeconds as crate::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0))
                    .expect("f02d58b1 pool_limits_and_timeouts_reject_zero invariant must hold"),
            );
        assert!(matches!(timeout, Err(super::PgPoolConfigParseError::Zero)));
    }
}
