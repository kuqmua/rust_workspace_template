#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractObservation {
    pub(super) body: crate::route_contract_validation::HttpContractBody,
    pub(super) metadata: frontend_contract::domain_types::RouteMetadata,
    pub(super) status: crate::route_contract_validation::HttpContractStatus,
}

impl HttpContractObservation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::domain_types::RouteMetadata,
        status: crate::route_contract_validation::HttpContractStatus,
        body: crate::route_contract_validation::HttpContractBody,
    ) -> Self {
        Self {
            body,
            metadata,
            status,
        }
    }
}
