#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ListTotalError {
    #[error("list total must not be negative")]
    Negative,
}
