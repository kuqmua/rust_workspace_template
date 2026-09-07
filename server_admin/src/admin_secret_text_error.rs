#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AdminSecretTextError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SECRET_TEXT_HAS_INVALID_BOUNDS)]
    InvalidBounds,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SECRET_TEXT_IS_TOO_SHORT)]
    TooShort,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SECRET_TEXT_IS_TOO_LONG)]
    TooLong,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SECRET_TEXT_CONTAINS_A_NUL_CHARACTER)]
    ContainsNul,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SECRET_TEXT_HAS_AN_INVALID_VALUE)]
    InvalidValue,
}

impl From<server_admin_core::std_admin_string::StdAdminStringTryFromStringError>
    for AdminSecretTextError
{
    fn from(value: server_admin_core::std_admin_string::StdAdminStringTryFromStringError) -> Self {
        match value {
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidBounds {
                ..
            } => Self::InvalidBounds,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooShort {
                ..
            } => Self::TooShort,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooLong {
                ..
            } => Self::TooLong,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::ContainsNul => {
                Self::ContainsNul
            }
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidValue => {
                Self::InvalidValue
            }
        }
    }
}
impl From<crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError>
    for AdminSecretTextError
{
    fn from(value: crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError) -> Self {
        match value {
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::ContainsNul => Self::ContainsNul,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidBounds { .. } => Self::InvalidBounds,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidValue => Self::InvalidValue,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooLong { .. } => Self::TooLong,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooShort { .. } => Self::TooShort,
        }
    }
}
