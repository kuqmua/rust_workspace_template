#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericBound {
    None,
    Inclusive(crate::contract_i64::ContractI64),
}
