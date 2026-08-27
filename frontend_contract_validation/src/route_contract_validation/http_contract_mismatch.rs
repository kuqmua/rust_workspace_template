#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum HttpContractMismatch {
    BodyExpectedEmpty,
    BodyExpectedJson,
    Metadata(super::RouteContractMismatches),
    Status {
        expected: super::HttpContractStatus,
        observed: super::HttpContractStatus,
    },
}
