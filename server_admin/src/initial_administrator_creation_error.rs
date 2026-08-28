#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum InitialAdministratorCreationError {
    #[error("initial administrator creation audit details are invalid")]
    AuditDetails,
    #[error("initial administrator creation display name is empty")]
    EmptyDisplayName,
    #[error("initial administrator creation login has an invalid format")]
    InvalidLogin,
    #[error("initial administrator creation password does not satisfy policy")]
    InvalidPassword,
    #[error("initial administrator creation has already been completed")]
    AlreadyInitialized,
    #[error("initial administrator creation password hashing failed: {0}")]
    PasswordHash(crate::AdminPasswordHashError),
    #[error("initial administrator creation database operation failed: {0:?}")]
    Pg(crate::SqlxAdminError),
}
