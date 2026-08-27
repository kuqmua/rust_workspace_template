#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractObservation {
    pub(super) body: super::HttpContractBody,
    pub(super) metadata: frontend_contract::domain_types::RouteMetadata,
    pub(super) status: super::HttpContractStatus,
}

impl HttpContractObservation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::domain_types::RouteMetadata,
        status: super::HttpContractStatus,
        body: super::HttpContractBody,
    ) -> Self {
        Self {
            body,
            metadata,
            status,
        }
    }
}
