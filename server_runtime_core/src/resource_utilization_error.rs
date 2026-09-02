#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum ResourceUtilizationError {
    #[error(
        "{}",
        constants_str::RESOURCE_UTILIZATION_MAXIMUM_MUST_BE_GREATER_THAN_ZERO
    )]
    ZeroMaximum,
}
