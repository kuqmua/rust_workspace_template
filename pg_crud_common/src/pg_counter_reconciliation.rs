#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgCounterReconciliation {
    ActualAhead(crate::pg_counter_value::PgCounterValue),
    InSync,
    TrackedAhead(crate::pg_counter_value::PgCounterValue),
}
