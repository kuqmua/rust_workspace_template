#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolMaxLifetimeSeconds(std::num::NonZeroU64);

impl crate::domain_types::TryFromStdEnvVarOk for PgPoolMaxLifetimeSeconds {
    type Error = super::PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: crate::domain_types::StdEnvVarOk) -> Result<Self, Self::Error> {
        super::parse_pg_pool_non_zero_seconds(&v).map(Self)
    }
}
