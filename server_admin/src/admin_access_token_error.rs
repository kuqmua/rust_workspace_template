#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminAccessTokenError {
    #[error("administrator access token has invalid bounds: {0}")]
    Bounds(crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError),
    #[error("administrator access token operation failed: {0:?}")]
    Token(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError),
}
impl From<crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError>
    for AdminAccessTokenError
{
    fn from(
        std_admin_access_token_try_from_string_error: crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError,
    ) -> Self {
        Self::Bounds(std_admin_access_token_try_from_string_error)
    }
}
impl From<crate::jsonwebtoken_admin_error::JsonwebtokenAdminError> for AdminAccessTokenError {
    fn from(
        jsonwebtoken_admin_error: crate::jsonwebtoken_admin_error::JsonwebtokenAdminError,
    ) -> Self {
        Self::Token(jsonwebtoken_admin_error)
    }
}
