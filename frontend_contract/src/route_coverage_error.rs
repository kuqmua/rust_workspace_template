#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteCoverageError {
    DuplicateRoute {
        metadata: crate::route_metadata::RouteMetadata,
    },
    Missing {
        metadata: crate::route_metadata::RouteMetadata,
        obligation: crate::route_coverage_obligation::RouteCoverageObligation,
    },
}
