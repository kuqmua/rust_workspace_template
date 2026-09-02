#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub enum FieldPlaceholder {
    None,
    Value(crate::contract_str::ContractStr),
}
