use super::TypedRoute;

pub trait CoveredRoute: TypedRoute {
    fn coverage_descriptor() -> crate::domain_types::RouteCoverageDescriptor;
}
