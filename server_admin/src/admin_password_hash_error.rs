#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminPasswordHashError {
    #[error("administrator password hashing task failed: {0:?}")]
    Join(crate::tokio_admin_join_error::TokioAdminJoinError),
    #[error("administrator password hashing failed: {0:?}")]
    PasswordHash(crate::argon2_admin_password_hash_error::Argon2AdminPasswordHashError),
    #[error("administrator password hashing concurrency limiter was closed: {0:?}")]
    SemaphoreClosed(crate::tokio_admin_acquire_error::TokioAdminAcquireError),
}
