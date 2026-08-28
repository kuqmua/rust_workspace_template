#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum HttpContractMismatch {
    BodyExpectedEmpty,
    BodyExpectedJson,
    Metadata(crate::route_contract_validation::RouteContractMismatches),
    Status {
        expected: crate::route_contract_validation::HttpContractStatus,
        observed: crate::route_contract_validation::HttpContractStatus,
    },
}
