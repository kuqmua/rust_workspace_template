#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminAccessTokenError {
    #[error("administrator access token operation failed: {0:?}")]
    Token(crate::jsonwebtoken_admin_error::JsonwebtokenAdminError),
}
impl From<crate::jsonwebtoken_admin_error::JsonwebtokenAdminError> for AdminAccessTokenError {
    fn from(value: crate::jsonwebtoken_admin_error::JsonwebtokenAdminError) -> Self {
        Self::Token(value)
    }
}
