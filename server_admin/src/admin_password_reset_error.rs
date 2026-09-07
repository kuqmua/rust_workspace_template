#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminPasswordResetError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_RESET_AUDIT_DETAILS_ARE_INVALID)]
    AuditDetails,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_RESET_LOGIN_HAS_AN_INVALID_FORMAT)]
    InvalidLogin,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_RESET_PASSWORD_DOES_NOT_SATISFY_POLICY)]
    InvalidPassword,
    #[error("{message}: {0}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_RESET_PASSWORD_HASHING_FAILED)]
    PasswordHash(crate::admin_password_hash_error::AdminPasswordHashError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_RESET_DATABASE_OPERATION_FAILED)]
    Pg(crate::sqlx_admin_error::SqlxAdminError),
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_RESET_TARGET_DOES_NOT_EXIST)]
    UnknownLogin,
}
