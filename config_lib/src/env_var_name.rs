use crate::{CONFIG_LIB_STRING_WRAPPER_MAX_LEN, ConfigLibStringWrapperTryFromStringError};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, newtype::Display,
)]
pub struct EnvVarName(String);
impl From<ConfigLibStringWrapperTryFromStringError> for EnvVarName {
    fn from(value: ConfigLibStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for EnvVarName {
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
