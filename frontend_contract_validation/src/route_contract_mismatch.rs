#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteContractMismatch {
    Method {
        expected: frontend_contract::ContractStr,
        observed: frontend_contract::ContractStr,
    },
    OpenApiOperationId {
        expected: frontend_contract::ContractStr,
        observed: frontend_contract::ContractStr,
    },
    Path {
        expected: frontend_contract::ContractStr,
        observed: frontend_contract::ContractStr,
    },
}
