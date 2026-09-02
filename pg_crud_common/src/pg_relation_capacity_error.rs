#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PgRelationCapacityError {
    #[error("PostgreSQL relation capacity would be exceeded")]
    Exceeded,
    #[error("PostgreSQL relation row count overflowed")]
    Overflow,
    #[error("PostgreSQL relation capacity maximum must be greater than zero")]
    ZeroMaximum,
}
