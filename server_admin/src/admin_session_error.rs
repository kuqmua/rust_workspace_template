#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminSessionError {
    #[error("administrator access token creation failed: {0:?}")]
    AccessToken(crate::admin_access_token_error::AdminAccessTokenError),
    #[error("administrator session database operation failed: {0:?}")]
    Pg(crate::sqlx_admin_error::SqlxAdminError),
    #[error("administrator session secret text is invalid: {0}")]
    SecretText(crate::admin_secret_text_error::AdminSecretTextError),
    #[error("system clock is before the Unix epoch")]
    SystemClock,
}
