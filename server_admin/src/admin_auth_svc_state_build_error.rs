#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error,
)]
pub enum AdminAuthSvcStateBuildError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_ALLOWED_ORIGIN_IS_INVALID)]
    AllowedOrigin,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_JWT_SECRET_LIST_IS_EMPTY)]
    JwtSecret,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_HASH_CONCURRENCY_IS_ZERO)]
    PasswordHashConcurrency,
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHENTICATION_NUMERIC_VALUE_IS_NOT_POSITIVE)]
    PositiveValue(#[source] crate::admin_auth_positive_value_error::AdminAuthPositiveValueError),
}
