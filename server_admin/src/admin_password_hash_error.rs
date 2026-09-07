#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminPasswordHashError {
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_HASHING_TASK_FAILED)]
    Join(crate::tokio_admin_join_error::TokioAdminJoinError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_HASHING_FAILED)]
    PasswordHash(crate::argon2_admin_password_hash_error::Argon2AdminPasswordHashError),
    #[error("{message}: {0:?}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_PASSWORD_HASHING_CONCURRENCY_LIMITER_WAS_CLOSED)]
    SemaphoreClosed(crate::tokio_admin_acquire_error::TokioAdminAcquireError),
}
