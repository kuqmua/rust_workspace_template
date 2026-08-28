use super::{StdEnvVarOk, TryFromStdEnvVarOk, types};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum TryFromStdEnvVarOkSvcModeError {
    #[error("service mode must be migrate or serve")]
    Unknown,
}
impl TryFromStdEnvVarOk for types::SvcMode {
    type Error = TryFromStdEnvVarOkSvcModeError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        match v.0.as_str() {
            constants_str::SERVICE_MODE_MIGRATE => Ok(Self::Migrate),
            constants_str::SERVICE_MODE_SERVE => Ok(Self::Serve),
            _unknown => Err(TryFromStdEnvVarOkSvcModeError::Unknown),
        }
    }
}
