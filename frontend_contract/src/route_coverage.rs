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
pub enum RouteDatabaseUsage {
    Database,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteJsonBodyUsage {
    JsonBody,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteResponseKind {
    Buffered,
    Streaming,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteTestCapabilities {
    database: RouteDatabaseUsage,
    json_body: RouteJsonBodyUsage,
    response: RouteResponseKind,
}

impl RouteTestCapabilities {
    #[must_use]
    pub const fn new(
        database: RouteDatabaseUsage,
        json_body: RouteJsonBodyUsage,
        response: RouteResponseKind,
    ) -> Self {
        Self {
            database,
            json_body,
            response,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteTestCategory {
    DatabaseFixture,
    FixtureHook,
    JsonRoundTrip,
    Metadata,
    StreamingResponse,
}
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct RouteTestCategories(bounded_types::BoundedVec<RouteTestCategory, 0, { usize::MAX }>);
impl From<Vec<RouteTestCategory>> for RouteTestCategories {
    fn from(value: Vec<RouteTestCategory>) -> Self {
        Self::from(bounded_types::BoundedVec::from_max_iter(value))
    }
}

#[must_use]
pub fn missing_required_test_categories(
    capabilities: RouteTestCapabilities,
    available_categories: &[RouteTestCategory],
) -> RouteTestCategories {
    RouteTestCategories::from(bounded_types::BoundedVec::from_max_iter(
        bounded_types::BoundedVec::from(required_test_categories(capabilities))
            .into_iter()
            .filter(|category| !available_categories.contains(category)),
    ))
}

#[must_use]
pub fn required_test_categories(capabilities: RouteTestCapabilities) -> RouteTestCategories {
    let categories = [
        Some(RouteTestCategory::FixtureHook),
        Some(RouteTestCategory::Metadata),
        (capabilities.database == RouteDatabaseUsage::Database)
            .then_some(RouteTestCategory::DatabaseFixture),
        (capabilities.json_body == RouteJsonBodyUsage::JsonBody)
            .then_some(RouteTestCategory::JsonRoundTrip),
        (capabilities.response == RouteResponseKind::Streaming)
            .then_some(RouteTestCategory::StreamingResponse),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    RouteTestCategories::from(bounded_types::BoundedVec::from_max_iter(categories))
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
    #[must_use]
    pub const fn metadata(self) -> crate::RouteMetadata {
        self.metadata
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
pub const PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
];
pub const PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
    RouteCoverageObligation::ReplayValidation,
];
pub const AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
    RouteCoverageObligation::SecurityValidation,
];
pub const AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
    RouteCoverageObligation::ReplayValidation,
    RouteCoverageObligation::SecurityValidation,
];

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
            crate::RouteMethod::Post,
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

    #[test]
    fn capabilities_require_matching_test_categories() {
        let capabilities = super::RouteTestCapabilities::new(
            super::RouteDatabaseUsage::Database,
            super::RouteJsonBodyUsage::JsonBody,
            super::RouteResponseKind::Streaming,
        );
        assert_eq!(
            bounded_types::BoundedVec::from(super::missing_required_test_categories(
                capabilities,
                &[
                    super::RouteTestCategory::FixtureHook,
                    super::RouteTestCategory::Metadata,
                ],
            ))
            .into_inner(),
            vec![
                super::RouteTestCategory::DatabaseFixture,
                super::RouteTestCategory::JsonRoundTrip,
                super::RouteTestCategory::StreamingResponse,
            ]
        );
    }

    #[test]
    fn routes_without_special_capabilities_require_baseline_categories() {
        let capabilities = super::RouteTestCapabilities::new(
            super::RouteDatabaseUsage::None,
            super::RouteJsonBodyUsage::None,
            super::RouteResponseKind::Buffered,
        );
        assert_eq!(
            bounded_types::BoundedVec::from(super::required_test_categories(capabilities))
                .into_inner(),
            vec![
                super::RouteTestCategory::FixtureHook,
                super::RouteTestCategory::Metadata,
            ]
        );
    }
}
