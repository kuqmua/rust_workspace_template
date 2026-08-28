#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
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
