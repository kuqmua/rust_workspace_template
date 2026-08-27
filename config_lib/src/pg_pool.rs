#[path = "pg_pool/parse_pg_pool_non_zero_seconds.rs"]
mod parse_pg_pool_non_zero_seconds;
#[path = "pg_pool/pg_pool_acquire_timeout_seconds.rs"]
mod pg_pool_acquire_timeout_seconds;
#[path = "pg_pool/pg_pool_config_parse_error.rs"]
mod pg_pool_config_parse_error;
#[path = "pg_pool/pg_pool_idle_timeout_seconds.rs"]
mod pg_pool_idle_timeout_seconds;
#[path = "pg_pool/pg_pool_max_connections.rs"]
mod pg_pool_max_connections;
#[path = "pg_pool/pg_pool_max_connections_try_from_u32_error.rs"]
mod pg_pool_max_connections_try_from_u32_error;
#[path = "pg_pool/pg_pool_max_lifetime_seconds.rs"]
mod pg_pool_max_lifetime_seconds;
#[path = "pg_pool/pg_pool_min_connections.rs"]
mod pg_pool_min_connections;
#[path = "pg_pool/request_timeout_seconds.rs"]
mod request_timeout_seconds;
#[path = "pg_pool/try_from_std_env_var_ok_pg_pool_max_connections_error.rs"]
mod try_from_std_env_var_ok_pg_pool_max_connections_error;

use parse_pg_pool_non_zero_seconds::parse_pg_pool_non_zero_seconds;
pub use pg_pool_acquire_timeout_seconds::PgPoolAcquireTimeoutSeconds;
pub use pg_pool_config_parse_error::PgPoolConfigParseError;
pub use pg_pool_idle_timeout_seconds::PgPoolIdleTimeoutSeconds;
pub use pg_pool_max_connections::*;
pub use pg_pool_max_connections_try_from_u32_error::PgPoolMaxConnectionsTryFromU32Error;
pub use pg_pool_max_lifetime_seconds::PgPoolMaxLifetimeSeconds;
pub use pg_pool_min_connections::PgPoolMinConnections;
pub use request_timeout_seconds::RequestTimeoutSeconds;
pub use try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError;

#[cfg(test)]
mod tests {
    #[test]
    fn pool_limits_and_timeouts_reject_zero() {
        let max = <super::PgPoolMaxConnections as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect("6f71a4b9 pool_limits_and_timeouts_reject_zero invariant must hold"),
        )
        .expect("c8ef416d pool_limits_and_timeouts_reject_zero invariant must hold");
        assert_eq!(max.0, 1u32);
        let timeout = <super::RequestTimeoutSeconds as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0)).expect("f02d58b1 pool_limits_and_timeouts_reject_zero invariant must hold"),
        );
        assert!(matches!(timeout, Err(super::PgPoolConfigParseError::Zero)));
    }
}
