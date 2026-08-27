#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct HttpContractExpectation {
    pub(super) metadata: frontend_contract::domain_types::RouteMetadata,
    pub(super) status: super::HttpContractStatus,
    pub(super) body_kind: super::HttpContractBodyKind,
}

impl HttpContractExpectation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::domain_types::RouteMetadata,
        status: super::HttpContractStatus,
        body_kind: super::HttpContractBodyKind,
    ) -> Self {
        Self {
            metadata,
            status,
            body_kind,
        }
    }
}
