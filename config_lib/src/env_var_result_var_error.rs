#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(super) struct EnvVarResultVarError(Result<String, std::env::VarError>);
impl TryFrom<Result<String, std::env::VarError>> for EnvVarResultVarError {
    type Error = crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError;
    fn try_from(result: Result<String, std::env::VarError>) -> Result<Self, Self::Error> {
        match result {
            Ok(raw_value) => {
                let bounded = crate::std_env_var_ok::StdEnvVarOk::try_from(raw_value)?;
                Ok(Self(Ok(String::from(bounded))))
            }
            Err(error) => Ok(Self(Err(error))),
        }
    }
}
