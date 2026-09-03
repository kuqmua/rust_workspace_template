#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_display::Display,
)]
pub struct EnvVarName(String);
impl From<crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError> for EnvVarName {
    fn from(config_lib_string_wrapper_try_from_string_error: crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError) -> Self {
        Self(config_lib_string_wrapper_try_from_string_error.to_string())
    }
}
impl TryFrom<String> for EnvVarName {
    type Error = crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len()
            > crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN
        {
            return Err(Self::Error::TooLong {
                len: string.len(),
                max: crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(string))
    }
}
