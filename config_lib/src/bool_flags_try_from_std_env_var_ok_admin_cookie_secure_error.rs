#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{0:?}")]
#[derive(newtype::FromInner)]
pub struct TryFromStdEnvVarOkAdminCookieSecureError(
    super::admin_bool_parsing_error::AdminBoolParsingError,
);
