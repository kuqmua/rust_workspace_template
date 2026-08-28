use super::LeaseId;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum LeaseReservation {
    Existing(LeaseId),
    LimitReached,
    Reserved,
}
