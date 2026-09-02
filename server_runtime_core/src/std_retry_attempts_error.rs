#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum StdRetryAttemptsError {
    #[error("retry attempts must be greater than zero")]
    Zero,
}
