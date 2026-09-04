#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct RouteCoverageDescriptor {
    #[constructor(order = 3)]
    evidence: crate::route_coverage_evidence::RouteCoverageEvidence,
    #[constructor(order = 0)]
    metadata: crate::route_metadata::RouteMetadata,
    #[constructor(order = 1)]
    access: crate::route_access::RouteAccess,
    #[constructor(order = 2)]
    mutation: crate::route_mutation::RouteMutation,
}
