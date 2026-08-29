#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolAcquireTimeoutSeconds(std::num::NonZeroU64);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for PgPoolAcquireTimeoutSeconds {
    type Error = crate::pg_pool_config_parse_error::PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        crate::parse_pg_pool_non_zero_seconds::parse_pg_pool_non_zero_seconds(&v).map(Self)
    }
}
