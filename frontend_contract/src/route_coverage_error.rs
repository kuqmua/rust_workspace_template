use super::RouteCoverageObligation;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteCoverageError {
    DuplicateRoute {
        metadata: crate::domain_types::RouteMetadata,
    },
    Missing {
        metadata: crate::domain_types::RouteMetadata,
        obligation: RouteCoverageObligation,
    },
}
