#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteContractMismatch {
    Method {
        expected: frontend_contract::domain_types::ContractStr,
        observed: frontend_contract::domain_types::ContractStr,
    },
    OpenApiOperationId {
        expected: frontend_contract::domain_types::ContractStr,
        observed: frontend_contract::domain_types::ContractStr,
    },
    Path {
        expected: frontend_contract::domain_types::ContractStr,
        observed: frontend_contract::domain_types::ContractStr,
    },
}
