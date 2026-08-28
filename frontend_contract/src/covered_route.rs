use super::TypedRoute;

pub trait CoveredRoute: TypedRoute {
    fn coverage_descriptor() -> crate::RouteCoverageDescriptor;
}
