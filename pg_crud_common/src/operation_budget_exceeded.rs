#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum OperationBudgetExceeded {
    #[error("operation count exceeds the deterministic budget")]
    Exceeded {
        actual: crate::operation_count::OperationCount,
        budget: crate::operation_budget::OperationBudget,
    },
}
