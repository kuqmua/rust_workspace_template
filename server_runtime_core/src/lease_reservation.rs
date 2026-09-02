#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum LeaseReservation {
    Existing(crate::lease_id::LeaseId),
    LimitReached,
    Reserved,
}
