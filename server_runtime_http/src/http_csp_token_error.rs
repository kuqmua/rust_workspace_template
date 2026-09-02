#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum HttpCspTokenError {
    #[error("content security policy token must not be empty")]
    Empty,
    #[error("content security policy token contains an invalid character")]
    InvalidCharacter,
    #[error("content security policy token is too long")]
    TooLong,
}
