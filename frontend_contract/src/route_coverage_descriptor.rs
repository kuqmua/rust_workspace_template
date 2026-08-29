#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteCoverageDescriptor {
    pub(super) evidence: crate::route_coverage_evidence::RouteCoverageEvidence,
    pub(super) metadata: crate::route_metadata::RouteMetadata,
    pub(super) access: crate::route_access::RouteAccess,
    pub(super) mutation: crate::route_mutation::RouteMutation,
}

impl RouteCoverageDescriptor {
    #[must_use]
    pub const fn new(
        metadata: crate::route_metadata::RouteMetadata,
        access: crate::route_access::RouteAccess,
        mutation: crate::route_mutation::RouteMutation,
        evidence: crate::route_coverage_evidence::RouteCoverageEvidence,
    ) -> Self {
        Self {
            evidence,
            metadata,
            access,
            mutation,
        }
    }
    #[must_use]
    pub const fn metadata(self) -> crate::route_metadata::RouteMetadata {
        self.metadata
    }
}
