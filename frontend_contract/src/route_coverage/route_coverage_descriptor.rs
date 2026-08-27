use super::{RouteAccess, RouteCoverageEvidence, RouteMutation};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteCoverageDescriptor {
    pub(super) evidence: RouteCoverageEvidence,
    pub(super) metadata: crate::domain_types::RouteMetadata,
    pub(super) access: RouteAccess,
    pub(super) mutation: RouteMutation,
}

impl RouteCoverageDescriptor {
    #[must_use]
    pub const fn new(
        metadata: crate::domain_types::RouteMetadata,
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
    pub const fn metadata(self) -> crate::domain_types::RouteMetadata {
        self.metadata
    }
}
