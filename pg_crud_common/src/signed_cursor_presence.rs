#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignedCursorPresence {
    Absent,
    Present,
}
