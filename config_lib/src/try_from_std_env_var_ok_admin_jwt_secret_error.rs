#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum TryFromStdEnvVarOkAdminJwtSecretError {
    #[error("administrator JWT secret list must not be empty")]
    Empty,
    #[error("administrator JWT secret list contains an empty entry")]
    EmptyEntry,
    #[error(
        "administrator JWT secret list must contain at most {} entries",
        super::ADMIN_JWT_SECRET_MAX_COUNT
    )]
    TooMany,
    #[error(
        "administrator JWT secret must contain at least {} bytes",
        super::ADMIN_JWT_SECRET_MIN_LEN
    )]
    TooShort,
    #[error("administrator JWT secret is too long")]
    TooLong,
}
