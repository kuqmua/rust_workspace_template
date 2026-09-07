#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminSessionError {
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_ACCESS_TOKEN_CREATION_FAILED)]
    AccessToken(crate::admin_access_token_error::AdminAccessTokenError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SESSION_DATABASE_OPERATION_FAILED)]
    Pg(crate::sqlx_admin_error::SqlxAdminError),
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_SESSION_SECRET_TEXT_IS_INVALID)]
    SecretText(crate::admin_secret_text_error::AdminSecretTextError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_SYSTEM_CLOCK_IS_BEFORE_THE_UNIX_EPOCH)]
    SystemClock,
}
