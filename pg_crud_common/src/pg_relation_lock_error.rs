#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PgRelationLockError {
    #[error("PostgreSQL relation lock namespace is invalid")]
    InvalidNamespace,
    #[error("PostgreSQL relation lock resource count exceeds 10000")]
    TooManyResources,
}
