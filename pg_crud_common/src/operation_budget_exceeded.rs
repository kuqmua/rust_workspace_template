#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("operation count exceeds the deterministic budget")]
pub struct OperationBudgetExceeded {
    actual: crate::domain_types::OperationCount,
    budget: crate::domain_types::OperationBudget,
}

impl OperationBudgetExceeded {
    pub(super) const fn new(
        actual: crate::domain_types::OperationCount,
        budget: crate::domain_types::OperationBudget,
    ) -> Self {
        Self { actual, budget }
    }
}
