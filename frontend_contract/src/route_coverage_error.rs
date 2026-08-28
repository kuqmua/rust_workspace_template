use super::RouteCoverageObligation;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteCoverageError {
    DuplicateRoute {
        metadata: crate::RouteMetadata,
    },
    Missing {
        metadata: crate::RouteMetadata,
        obligation: RouteCoverageObligation,
    },
}
