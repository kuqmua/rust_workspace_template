#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminSessionError {
    #[error("administrator access token creation failed: {0:?}")]
    AccessToken(crate::AdminAccessTokenError),
    #[error("administrator session database operation failed: {0:?}")]
    Pg(crate::SqlxAdminError),
    #[error("administrator session secret text is invalid: {0}")]
    SecretText(crate::AdminSecretTextError),
    #[error("system clock is before the Unix epoch")]
    SystemClock,
}
