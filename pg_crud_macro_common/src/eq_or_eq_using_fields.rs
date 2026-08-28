#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOrEqUsingFields {
    Eq,
    EqUsingFields,
}
