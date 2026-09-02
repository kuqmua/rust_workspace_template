#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum CursorSigningKeyError {
    #[error("{message}", message = constants_str::CURSOR_SIGNING_KEY_LENGTH_INVALID)]
    InvalidLength,
}
