#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct PgPoolMinConnections(u32);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for PgPoolMinConnections {
    type Error = crate::pg_pool_config_parse_error::PgPoolConfigParseError;
    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        std_env_var_ok
            .parse::<u32>()
            .map(Self)
            .map_err(|_error| Self::Error::Parse)
    }
}
