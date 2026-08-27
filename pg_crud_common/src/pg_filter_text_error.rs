#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("PostgreSQL filter text exceeds its maximum size")]
pub struct PgFilterTextError;
