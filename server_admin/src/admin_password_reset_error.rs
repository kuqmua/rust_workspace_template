#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminPasswordResetError {
    #[error("administrator password reset audit details are invalid")]
    AuditDetails,
    #[error("administrator password reset login has an invalid format")]
    InvalidLogin,
    #[error("administrator password reset password does not satisfy policy")]
    InvalidPassword,
    #[error("administrator password reset password hashing failed: {0}")]
    PasswordHash(super::AdminPasswordHashError),
    #[error("administrator password reset database operation failed: {0:?}")]
    Pg(super::SqlxAdminError),
    #[error("administrator password reset target does not exist")]
    UnknownLogin,
}
