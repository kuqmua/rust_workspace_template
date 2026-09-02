#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum SqlIdentifierError {
    #[error("SQL identifier is empty")]
    Empty,
    #[error("SQL identifier contains unsupported characters")]
    Invalid,
}
