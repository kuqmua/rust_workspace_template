#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error(
    "{}",
    constants_str::catalog::RESOURCE_BUDGET_MAXIMUM_MUST_BE_GREATER_THAN_ZERO
)]
pub struct ResourceBudgetConfigError;
