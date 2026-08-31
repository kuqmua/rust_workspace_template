pub fn validate_route_coverage(
    descriptors: &[crate::route_coverage_descriptor::RouteCoverageDescriptor],
) -> Result<(), crate::route_coverage_error::RouteCoverageError> {
    descriptors
        .iter()
        .enumerate()
        .try_for_each(|(index, descriptor)| {
            if descriptors
                .iter()
                .take(index)
                .any(|previous| previous.get_metadata() == descriptor.get_metadata())
            {
                return Err(crate::route_coverage_error::RouteCoverageError::DuplicateRoute {
                    metadata: *descriptor.get_metadata(),
                });
            }
            let required = [
                (
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                    descriptor.get_evidence().get_obligations().contains(
                        &crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                    ),
                ),
                (
                    crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
                    descriptor.get_evidence().get_obligations().contains(
                        &crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
                    ),
                ),
                (
                    crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
                    descriptor.get_evidence().get_obligations().contains(
                        &crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
                    ),
                ),
            ];
            if let Some((obligation, _present)) =
                required.into_iter().find(|(_kind, present)| !present)
            {
                return Err(crate::route_coverage_error::RouteCoverageError::Missing {
                    metadata: *descriptor.get_metadata(),
                    obligation,
                });
            }
            if *descriptor.get_access() == crate::route_access::RouteAccess::Authenticated
                && !descriptor
                    .get_evidence()
                    .get_obligations()
                    .contains(&crate::route_coverage_obligation::RouteCoverageObligation::SecurityValidation)
            {
                return Err(crate::route_coverage_error::RouteCoverageError::Missing {
                    metadata: *descriptor.get_metadata(),
                    obligation: crate::route_coverage_obligation::RouteCoverageObligation::SecurityValidation,
                });
            }
            if *descriptor.get_mutation() == crate::route_mutation::RouteMutation::Mutating
                && !descriptor
                    .get_evidence()
                    .get_obligations()
                    .contains(&crate::route_coverage_obligation::RouteCoverageObligation::ReplayValidation)
            {
                return Err(crate::route_coverage_error::RouteCoverageError::Missing {
                    metadata: *descriptor.get_metadata(),
                    obligation: crate::route_coverage_obligation::RouteCoverageObligation::ReplayValidation,
                });
            }
            Ok(())
        })
}
