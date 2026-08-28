#[derive(
    Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PgPoolMaxConnectionsTryFromU32Error {
    #[error("pg pool max connections must be greater than zero")]
    IsZero,
}
