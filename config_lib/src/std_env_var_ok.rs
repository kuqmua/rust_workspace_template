#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct StdEnvVarOk(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN },
        false,
    >,
);
impl From<crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError> for StdEnvVarOk {
    fn from(value: crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError) -> Self {
        bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(value)
    }
}
impl TryFrom<String> for StdEnvVarOk {
    type Error = crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN
        {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            });
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    maximum_length,
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    actual_length,
                    minimum_length: maximum_length,
                } => Self::Error::TooLong {
                    len: actual_length.get(),
                    max: maximum_length.get(),
                },
            })
    }
}
