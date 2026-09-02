#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum DevelopmentIdentitySpecsError {
    #[error("{self:?}")]
    TooMany,
}
