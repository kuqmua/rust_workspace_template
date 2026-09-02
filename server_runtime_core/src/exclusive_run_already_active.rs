#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum ExclusiveRunAlreadyActive {
    #[error("operation is already running")]
    Active,
}
