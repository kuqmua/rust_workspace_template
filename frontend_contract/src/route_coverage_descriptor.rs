#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
)]
pub struct RouteCoverageDescriptor {
    evidence: crate::route_coverage_evidence::RouteCoverageEvidence,
    metadata: crate::route_metadata::RouteMetadata,
    access: crate::route_access::RouteAccess,
    mutation: crate::route_mutation::RouteMutation,
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
}
