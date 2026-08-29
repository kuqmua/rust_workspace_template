pub trait RouteFamily {
    const ROUTE_COUNT: usize = constants_usize::ZERO;
    #[must_use]
    fn body_limit() -> Option<crate::route_body_limit::RouteBodyLimit> {
        None
    }
    fn coverage_descriptors() -> crate::route_coverage_descriptors::RouteCoverageDescriptors;
    #[must_use]
    fn schema_contracts() -> crate::route_schema_contracts::RouteSchemaContracts {
        crate::route_schema_contracts::RouteSchemaContracts::default()
    }
    fn route_metadata() -> crate::route_metadata_list::RouteMetadataList {
        crate::route_metadata_list::RouteMetadataList::from(
            bounded_types::bounded_vec::BoundedVec::from_max_iter(
                bounded_types::bounded_vec::BoundedVec::from(Self::coverage_descriptors())
                    .into_iter()
                    .map(crate::route_coverage_descriptor::RouteCoverageDescriptor::metadata),
            ),
        )
    }
}
