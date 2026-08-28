use super::{RouteBodyLimit, RouteCoverageDescriptors, RouteMetadataList, RouteSchemaContracts};

pub trait RouteFamily {
    const ROUTE_COUNT: usize = constants_usize::ZERO;
    #[must_use]
    fn body_limit() -> Option<RouteBodyLimit> {
        None
    }
    fn coverage_descriptors() -> RouteCoverageDescriptors;
    #[must_use]
    fn schema_contracts() -> RouteSchemaContracts {
        RouteSchemaContracts::default()
    }
    fn route_metadata() -> RouteMetadataList {
        RouteMetadataList::from(bounded_types::BoundedVec::from_max_iter(
            bounded_types::BoundedVec::from(Self::coverage_descriptors())
                .into_iter()
                .map(crate::RouteCoverageDescriptor::metadata),
        ))
    }
}
