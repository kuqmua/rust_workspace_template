#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum TryFromStdEnvVarOkSvcModeError {
    #[error("service mode must be migrate or serve")]
    Unknown,
}
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for crate::svc_mode::SvcMode {
    type Error = TryFromStdEnvVarOkSvcModeError;
    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        match std_env_var_ok.as_str() {
            constants_str::SERVICE_MODE_MIGRATE => Ok(Self::Migrate),
            constants_str::SERVICE_MODE_SERVE => Ok(Self::Serve),
            _unknown => Err(TryFromStdEnvVarOkSvcModeError::Unknown),
        }
    }
}
