#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractObservation {
    pub(super) body: crate::http_contract_body::HttpContractBody,
    pub(super) metadata: frontend_contract::route_metadata::RouteMetadata,
    pub(super) status: crate::http_contract_status::HttpContractStatus,
}

impl HttpContractObservation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::route_metadata::RouteMetadata,
        status: crate::http_contract_status::HttpContractStatus,
        body: crate::http_contract_body::HttpContractBody,
    ) -> Self {
        Self {
            body,
            metadata,
            status,
        }
    }
}
