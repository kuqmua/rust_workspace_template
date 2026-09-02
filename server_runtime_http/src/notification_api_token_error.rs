#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum NotificationApiTokenError {
    #[error("notification API token must not be empty")]
    Empty,
    #[error("notification API token exceeds maximum length")]
    TooLong,
}
