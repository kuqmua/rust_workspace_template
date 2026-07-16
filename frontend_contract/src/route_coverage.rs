#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteAccess {
    Authenticated,
    Public,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteMutation {
    Mutating,
    ReadOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteCoverageEvidence {
    obligations: &'static [RouteCoverageObligation],
}

impl RouteCoverageEvidence {
    #[must_use]
    pub const fn new(obligations: &'static [RouteCoverageObligation]) -> Self {
        Self { obligations }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteCoverageDescriptor {
    access: RouteAccess,
    evidence: RouteCoverageEvidence,
    metadata: crate::RouteMetadata,
    mutation: RouteMutation,
}

impl RouteCoverageDescriptor {
    #[must_use]
    pub const fn new(
        metadata: crate::RouteMetadata,
        access: RouteAccess,
        mutation: RouteMutation,
        evidence: RouteCoverageEvidence,
    ) -> Self {
        Self {
            access,
            evidence,
            metadata,
            mutation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteCoverageObligation {
    IntegrationFixture,
    OpenApiOperation,
    PayloadValidation,
    ReplayValidation,
    SecurityValidation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteCoverageError {
    DuplicateRoute {
        metadata: crate::RouteMetadata,
    },
    Missing {
        metadata: crate::RouteMetadata,
        obligation: RouteCoverageObligation,
    },
}

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

#[cfg(test)]
mod tests {
    fn metadata() -> crate::RouteMetadata {
        crate::RouteMetadata::new(
            str_constants::POST.into(),
            str_constants::ROUTE_READ.into(),
            str_constants::ROUTE.into(),
        )
    }

    #[test]
    fn complete_mutating_authenticated_route_is_covered() {
        let descriptors = [super::RouteCoverageDescriptor::new(
            metadata(),
            super::RouteAccess::Authenticated,
            super::RouteMutation::Mutating,
            super::RouteCoverageEvidence::new(&[
                super::RouteCoverageObligation::IntegrationFixture,
                super::RouteCoverageObligation::OpenApiOperation,
                super::RouteCoverageObligation::PayloadValidation,
                super::RouteCoverageObligation::ReplayValidation,
                super::RouteCoverageObligation::SecurityValidation,
            ]),
        )];
        assert_eq!(super::validate_route_coverage(&descriptors), Ok(()));
    }

    #[test]
    fn mutating_route_requires_replay_validation() {
        let descriptors = [super::RouteCoverageDescriptor::new(
            metadata(),
            super::RouteAccess::Public,
            super::RouteMutation::Mutating,
            super::RouteCoverageEvidence::new(&[
                super::RouteCoverageObligation::IntegrationFixture,
                super::RouteCoverageObligation::OpenApiOperation,
                super::RouteCoverageObligation::PayloadValidation,
            ]),
        )];
        assert!(matches!(
            super::validate_route_coverage(&descriptors),
            Err(super::RouteCoverageError::Missing {
                obligation: super::RouteCoverageObligation::ReplayValidation,
                ..
            })
        ));
    }

    #[test]
    fn duplicate_route_is_rejected() {
        let descriptor = super::RouteCoverageDescriptor::new(
            metadata(),
            super::RouteAccess::Public,
            super::RouteMutation::ReadOnly,
            super::RouteCoverageEvidence::new(&[
                super::RouteCoverageObligation::IntegrationFixture,
                super::RouteCoverageObligation::OpenApiOperation,
                super::RouteCoverageObligation::PayloadValidation,
            ]),
        );
        assert!(matches!(
            super::validate_route_coverage(&[descriptor, descriptor]),
            Err(super::RouteCoverageError::DuplicateRoute { .. })
        ));
    }
}
