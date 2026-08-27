#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct EnvVarResultVarError(pub(super) Result<String, std::env::VarError>);
impl TryFrom<Result<String, std::env::VarError>> for EnvVarResultVarError {
    type Error = super::ConfigLibStringWrapperTryFromStringError;
    fn try_from(value: Result<String, std::env::VarError>) -> Result<Self, Self::Error> {
        match value {
            Ok(raw_value) => {
                let bounded = super::StdEnvVarOk::try_from(raw_value)?;
                Ok(Self(Ok(bounded.0)))
            }
            Err(error) => Ok(Self(Err(error))),
        }
    }
}
