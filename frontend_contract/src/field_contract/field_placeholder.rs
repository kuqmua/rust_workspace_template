use super::ContractStr;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPlaceholder {
    None,
    Value(ContractStr),
}
