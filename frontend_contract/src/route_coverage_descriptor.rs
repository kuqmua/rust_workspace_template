#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
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
        route_metadata: crate::route_metadata::RouteMetadata,
        route_access: crate::route_access::RouteAccess,
        route_mutation: crate::route_mutation::RouteMutation,
        route_coverage_evidence: crate::route_coverage_evidence::RouteCoverageEvidence,
    ) -> Self {
        Self {
            evidence: route_coverage_evidence,
            metadata: route_metadata,
            access: route_access,
            mutation: route_mutation,
        }
    }
}
