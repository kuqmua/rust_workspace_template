#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum NotificationMessageError {
    #[error("notification message must not be empty")]
    Empty,
    #[error("notification message exceeds maximum length")]
    TooLong,
}
