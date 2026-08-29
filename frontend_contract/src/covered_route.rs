pub trait CoveredRoute: crate::typed_route::TypedRoute {
    fn coverage_descriptor() -> crate::route_coverage_descriptor::RouteCoverageDescriptor;
}
