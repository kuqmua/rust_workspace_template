use super::{
    RouteAccess, RouteCoverageDescriptor, RouteCoverageError, RouteCoverageObligation,
    RouteMutation,
};

pub fn validate_route_coverage(
    descriptors: &[RouteCoverageDescriptor],
) -> Result<(), RouteCoverageError> {
    descriptors
        .iter()
        .enumerate()
        .try_for_each(|(index, descriptor)| {
            if descriptors
                .iter()
                .take(index)
                .any(|previous| previous.metadata == descriptor.metadata)
            {
                return Err(RouteCoverageError::DuplicateRoute {
                    metadata: descriptor.metadata,
                });
            }
            let required = [
                (
                    RouteCoverageObligation::IntegrationFixture,
                    descriptor
                        .evidence
                        .obligations
                        .contains(&RouteCoverageObligation::IntegrationFixture),
                ),
                (
                    RouteCoverageObligation::OpenApiOperation,
                    descriptor
                        .evidence
                        .obligations
                        .contains(&RouteCoverageObligation::OpenApiOperation),
                ),
                (
                    RouteCoverageObligation::PayloadValidation,
                    descriptor
                        .evidence
                        .obligations
                        .contains(&RouteCoverageObligation::PayloadValidation),
                ),
            ];
            if let Some((obligation, _present)) =
                required.into_iter().find(|(_kind, present)| !present)
            {
                return Err(RouteCoverageError::Missing {
                    metadata: descriptor.metadata,
                    obligation,
                });
            }
            if descriptor.access == RouteAccess::Authenticated
                && !descriptor
                    .evidence
                    .obligations
                    .contains(&RouteCoverageObligation::SecurityValidation)
            {
                return Err(RouteCoverageError::Missing {
                    metadata: descriptor.metadata,
                    obligation: RouteCoverageObligation::SecurityValidation,
                });
            }
            if descriptor.mutation == RouteMutation::Mutating
                && !descriptor
                    .evidence
                    .obligations
                    .contains(&RouteCoverageObligation::ReplayValidation)
            {
                return Err(RouteCoverageError::Missing {
                    metadata: descriptor.metadata,
                    obligation: RouteCoverageObligation::ReplayValidation,
                });
            }
            Ok(())
        })
}
