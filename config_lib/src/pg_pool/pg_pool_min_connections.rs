#[derive(
    Debug,
    Clone,
    Copy,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolMinConnections(u32);

impl crate::domain_types::TryFromStdEnvVarOk for PgPoolMinConnections {
    type Error = super::PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: crate::domain_types::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<u32>()
            .map(Self)
            .map_err(|_error| Self::Error::Parse)
    }
}
