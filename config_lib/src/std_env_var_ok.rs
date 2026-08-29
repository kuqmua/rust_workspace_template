#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{CONFIG_LIB_STRING_WRAPPER_MAX_LEN, ConfigLibStringWrapperTryFromStringError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq)]
pub struct StdEnvVarOk(pub(super) String);
impl From<ConfigLibStringWrapperTryFromStringError> for StdEnvVarOk {
    fn from(value: ConfigLibStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for StdEnvVarOk {
    type Error = ConfigLibStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > CONFIG_LIB_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
