#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum HttpContractMismatch {
    BodyExpectedEmpty,
    BodyExpectedJson,
    Metadata(crate::route_contract_mismatches::RouteContractMismatches),
    Status {
        expected: crate::http_contract_status::HttpContractStatus,
        observed: crate::http_contract_status::HttpContractStatus,
    },
}
