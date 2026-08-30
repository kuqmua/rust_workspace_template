#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::IntoInnerFrom)]
pub(super) struct EnvVarResultVarError(Result<String, std::env::VarError>);
impl TryFrom<Result<String, std::env::VarError>> for EnvVarResultVarError {
    type Error = crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError;
    fn try_from(value: Result<String, std::env::VarError>) -> Result<Self, Self::Error> {
        match value {
            Ok(raw_value) => {
                let bounded = crate::std_env_var_ok::StdEnvVarOk::try_from(raw_value)?;
                Ok(Self(Ok(String::from(bounded))))
            }
            Err(error) => Ok(Self(Err(error))),
        }
    }
}
