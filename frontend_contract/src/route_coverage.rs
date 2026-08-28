pub use crate::missing_required_test_categories::missing_required_test_categories;
pub use crate::required_test_categories::required_test_categories;
pub use crate::route_access::RouteAccess;
pub use crate::route_coverage_descriptor::RouteCoverageDescriptor;
pub use crate::route_coverage_error::RouteCoverageError;
pub use crate::route_coverage_evidence::RouteCoverageEvidence;
pub use crate::route_coverage_obligation::*;
pub use crate::route_database_usage::RouteDatabaseUsage;
pub use crate::route_json_body_usage::RouteJsonBodyUsage;
pub use crate::route_mutation::RouteMutation;
pub use crate::route_response_kind::RouteResponseKind;
pub use crate::route_test_capabilities::RouteTestCapabilities;
pub use crate::route_test_categories::RouteTestCategories;
pub use crate::route_test_category::RouteTestCategory;
pub use crate::validate_route_coverage::validate_route_coverage;

#[cfg(test)]
mod tests {
    fn route_coverage_metadata() -> crate::RouteMetadata {
        crate::RouteMetadata::new(
            crate::RouteMethod::Post,
            constants_str::ROUTE_READ.into(),
            constants_str::ROUTE.into(),
        )
    }

    #[test]
    fn complete_mutating_authenticated_route_is_covered() {
        let descriptors = [super::RouteCoverageDescriptor::new(
            route_coverage_metadata(),
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
            route_coverage_metadata(),
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
            route_coverage_metadata(),
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
