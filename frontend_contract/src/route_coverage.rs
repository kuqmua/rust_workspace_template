#[path = "route_coverage/missing_required_test_categories.rs"]
mod missing_required_test_categories;
#[path = "route_coverage/required_test_categories.rs"]
mod required_test_categories;
#[path = "route_coverage/route_access.rs"]
mod route_access;
#[path = "route_coverage/route_coverage_descriptor.rs"]
mod route_coverage_descriptor;
#[path = "route_coverage/route_coverage_error.rs"]
mod route_coverage_error;
#[path = "route_coverage/route_coverage_evidence.rs"]
mod route_coverage_evidence;
#[path = "route_coverage/route_coverage_obligation.rs"]
mod route_coverage_obligation;
#[path = "route_coverage/route_database_usage.rs"]
mod route_database_usage;
#[path = "route_coverage/route_json_body_usage.rs"]
mod route_json_body_usage;
#[path = "route_coverage/route_mutation.rs"]
mod route_mutation;
#[path = "route_coverage/route_response_kind.rs"]
mod route_response_kind;
#[path = "route_coverage/route_test_capabilities.rs"]
mod route_test_capabilities;
#[path = "route_coverage/route_test_categories.rs"]
mod route_test_categories;
#[path = "route_coverage/route_test_category.rs"]
mod route_test_category;
#[path = "route_coverage/validate_route_coverage.rs"]
mod validate_route_coverage;

pub use missing_required_test_categories::missing_required_test_categories;
pub use required_test_categories::required_test_categories;
pub use route_access::RouteAccess;
pub use route_coverage_descriptor::RouteCoverageDescriptor;
pub use route_coverage_error::RouteCoverageError;
pub use route_coverage_evidence::RouteCoverageEvidence;
pub use route_coverage_obligation::*;
pub use route_database_usage::RouteDatabaseUsage;
pub use route_json_body_usage::RouteJsonBodyUsage;
pub use route_mutation::RouteMutation;
pub use route_response_kind::RouteResponseKind;
pub use route_test_capabilities::RouteTestCapabilities;
pub use route_test_categories::RouteTestCategories;
pub use route_test_category::RouteTestCategory;
pub use validate_route_coverage::validate_route_coverage;

#[cfg(test)]
mod tests {
    fn metadata() -> crate::domain_types::RouteMetadata {
        crate::domain_types::RouteMetadata::new(
            crate::domain_types::RouteMethod::Post,
            constants_str::ROUTE_READ.into(),
            constants_str::ROUTE.into(),
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
            bounded_types::domain_types::vector::BoundedVec::from(
                super::missing_required_test_categories(
                    capabilities,
                    &[
                        super::RouteTestCategory::FixtureHook,
                        super::RouteTestCategory::Metadata,
                    ],
                )
            )
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
            bounded_types::domain_types::vector::BoundedVec::from(super::required_test_categories(
                capabilities
            ))
            .into_inner(),
            vec![
                super::RouteTestCategory::FixtureHook,
                super::RouteTestCategory::Metadata,
            ]
        );
    }
}
