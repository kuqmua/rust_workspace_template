#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum InitialAdministratorCreationError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_AUDIT_DETAILS_ARE_INVALID)]
    AuditDetails,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_DISPLAY_NAME_IS_EMPTY)]
    EmptyDisplayName,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_LOGIN_HAS_AN_INVALID_FORMAT)]
    InvalidLogin,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_PASSWORD_DOES_NOT_SATISFY_POLICY)]
    InvalidPassword,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_HAS_ALREADY_BEEN_COMPLETED)]
    AlreadyInitialized,
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_PASSWORD_HASHING_FAILED)]
    PasswordHash(crate::admin_password_hash_error::AdminPasswordHashError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_INITIAL_ADMINISTRATOR_CREATION_DATABASE_OPERATION_FAILED)]
    Pg(crate::sqlx_admin_error::SqlxAdminError),
}
