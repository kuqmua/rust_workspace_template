#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct RequestTimeoutSeconds(std::num::NonZeroU64);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for RequestTimeoutSeconds {
    type Error = crate::pg_pool_config_parse_error::PgPoolConfigParseError;
    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        crate::parse_pg_pool_non_zero_seconds::parse_pg_pool_non_zero_seconds(&std_env_var_ok)
            .map(Self)
    }
}
