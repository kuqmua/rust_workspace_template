#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq, thiserror::Error,
)]
pub(super) enum EnvParseError {
    #[error("environment variable value exceeds the size limit")]
    ValueTooLong {
        #[source]
        source: crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError,
    },
    #[error("std::env::var(\"{name}\")")]
    Read {
        name: crate::env_var_name::EnvVarName,
        #[source]
        source: crate::env_var_error::EnvVarError,
    },
    #[error("{context}: {detail}")]
    Parse {
        context: crate::parse_context_ref::ParseContextRef,
        detail: to_err_string::error_text::ErrorText,
    },
}
impl From<crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError> for EnvParseError {
    fn from(config_lib_string_wrapper_try_from_string_error: crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError) -> Self {
        Self::ValueTooLong { source: config_lib_string_wrapper_try_from_string_error }
    }
}
