#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum RouteContractMismatch {
    Method {
        expected: frontend_contract::contract_str::ContractStr,
        observed: frontend_contract::contract_str::ContractStr,
    },
    OpenApiOperationId {
        expected: frontend_contract::contract_str::ContractStr,
        observed: frontend_contract::contract_str::ContractStr,
    },
    Path {
        expected: frontend_contract::contract_str::ContractStr,
        observed: frontend_contract::contract_str::ContractStr,
    },
}
