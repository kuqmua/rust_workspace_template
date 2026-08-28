#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    generate_constructor::New,
)]
#[error("operation count exceeds the deterministic budget")]
#[constructor(pub(super))]
pub struct OperationBudgetExceeded {
    actual: crate::domain_types::OperationCount,
    budget: crate::domain_types::OperationBudget,
}
