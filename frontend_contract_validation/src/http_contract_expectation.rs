#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct HttpContractExpectation {
    pub(super) metadata: frontend_contract::route_metadata::RouteMetadata,
    pub(super) status: crate::http_contract_status::HttpContractStatus,
    pub(super) body_kind: crate::http_contract_body_kind::HttpContractBodyKind,
}

impl HttpContractExpectation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::route_metadata::RouteMetadata,
        status: crate::http_contract_status::HttpContractStatus,
        body_kind: crate::http_contract_body_kind::HttpContractBodyKind,
    ) -> Self {
        Self {
            metadata,
            status,
            body_kind,
        }
    }
}
