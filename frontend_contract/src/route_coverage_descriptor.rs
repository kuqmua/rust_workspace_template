#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{RouteAccess, RouteCoverageEvidence, RouteMutation};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteCoverageDescriptor {
    pub(super) evidence: RouteCoverageEvidence,
    pub(super) metadata: crate::RouteMetadata,
    pub(super) access: RouteAccess,
    pub(super) mutation: RouteMutation,
}

impl RouteCoverageDescriptor {
    #[must_use]
    pub const fn new(
        metadata: crate::RouteMetadata,
        access: RouteAccess,
        mutation: RouteMutation,
        evidence: RouteCoverageEvidence,
    ) -> Self {
        Self {
            evidence,
            metadata,
            access,
            mutation,
        }
    }
    #[must_use]
    pub const fn metadata(self) -> crate::RouteMetadata {
        self.metadata
    }
}
