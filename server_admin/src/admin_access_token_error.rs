#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminAccessTokenError {
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_ACCESS_TOKEN_HAS_INVALID_BOUNDS)]
    Bounds(crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_ACCESS_TOKEN_OPERATION_FAILED)]
    Token(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError),
}
impl From<crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError>
    for AdminAccessTokenError
{
    fn from(value: crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError) -> Self {
        Self::Bounds(value)
    }
}
impl From<crate::jsonwebtoken_admin_error::JsonwebtokenAdminError> for AdminAccessTokenError {
    fn from(value: crate::jsonwebtoken_admin_error::JsonwebtokenAdminError) -> Self {
        Self::Token(value)
    }
}
