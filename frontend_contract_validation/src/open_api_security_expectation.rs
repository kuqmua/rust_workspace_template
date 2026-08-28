#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenApiSecurityExpectation {
    Public,
    Required(frontend_contract::ContractStr),
}
