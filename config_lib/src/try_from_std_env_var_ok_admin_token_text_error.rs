#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum TryFromStdEnvVarOkAdminTokenTextError {
    #[error("administrator token text is empty")]
    Empty,
    #[error("administrator token text is too long")]
    TooLong,
}
