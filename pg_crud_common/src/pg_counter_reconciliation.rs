#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgCounterReconciliation {
    ActualAhead(crate::domain_types::PgCounterValue),
    InSync,
    TrackedAhead(crate::domain_types::PgCounterValue),
}
