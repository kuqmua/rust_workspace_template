#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum ResourceBudgetConfigError {
    #[error("{}", constants_str::RESOURCE_BUDGET_MAXIMUM_MUST_BE_GREATER_THAN_ZERO)]
    Zero,
}
