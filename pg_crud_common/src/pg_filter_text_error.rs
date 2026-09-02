#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum PgFilterTextError {
    #[error("PostgreSQL filter text exceeds its maximum size")]
    TooLarge,
}
