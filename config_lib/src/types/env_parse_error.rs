use super::{EnvVarError, ParseCtxRef};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq, thiserror::Error)]
pub(super) enum EnvParseError {
    #[error("environment variable value exceeds the size limit")]
    ValueTooLong {
        #[source]
        source: super::ConfigLibStringWrapperTryFromStringError,
    },
    #[error("std::env::var(\"{name}\")")]
    Read {
        name: super::EnvVarName,
        #[source]
        source: EnvVarError,
    },
    #[error("{context}: {detail}")]
    Parse {
        context: ParseCtxRef,
        detail: to_err_string::domain_types::ErrorText,
    },
}
impl From<super::ConfigLibStringWrapperTryFromStringError> for EnvParseError {
    fn from(value: super::ConfigLibStringWrapperTryFromStringError) -> Self {
        Self::ValueTooLong { source: value }
    }
}
