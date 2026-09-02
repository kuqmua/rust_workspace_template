#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum ResourceBudgetReserveError {
    #[error("{}", constants_str::RESOURCE_BUDGET_EXHAUSTED)]
    Exhausted,
    #[error("{}", constants_str::RESOURCE_BUDGET_RESERVATION_OVERFLOW)]
    Overflow,
}
