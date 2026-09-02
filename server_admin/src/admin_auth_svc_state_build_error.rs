#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error,
)]
pub enum AdminAuthSvcStateBuildError {
    #[error("administrator allowed origin is invalid")]
    AllowedOrigin,
    #[error("administrator JWT secret list is empty")]
    JwtSecret,
    #[error("administrator password hash concurrency is zero")]
    PasswordHashConcurrency,
    #[error("administrator authentication numeric value is not positive")]
    PositiveValue(#[source] crate::admin_auth_positive_value_error::AdminAuthPositiveValueError),
}
