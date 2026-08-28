#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticationRequirement {
    Authenticated,
    Permission(crate::ContractStr),
    Public,
}
