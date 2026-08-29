#[cfg(test)]
mod tests {
    fn route_coverage_metadata() -> crate::route_metadata::RouteMetadata {
        crate::route_metadata::RouteMetadata::new(
            crate::route_method::RouteMethod::Post,
            constants_str::catalog::ROUTE_READ.into(),
            constants_str::catalog::ROUTE.into(),
        )
    }

    #[test]
    fn complete_mutating_authenticated_route_is_covered() {
        let descriptors = [
            crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
                route_coverage_metadata(),
                crate::route_access::RouteAccess::Authenticated,
                crate::route_mutation::RouteMutation::Mutating,
                crate::route_coverage_evidence::RouteCoverageEvidence::new(&[
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                    crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
                    crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
                    crate::route_coverage_obligation::RouteCoverageObligation::ReplayValidation,
                    crate::route_coverage_obligation::RouteCoverageObligation::SecurityValidation,
                ]),
            ),
        ];
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&descriptors),
            Ok(())
        );
    }

    #[test]
    fn mutating_route_requires_replay_validation() {
        let descriptors = [
            crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
                route_coverage_metadata(),
                crate::route_access::RouteAccess::Public,
                crate::route_mutation::RouteMutation::Mutating,
                crate::route_coverage_evidence::RouteCoverageEvidence::new(&[
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                    crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
                    crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
                ]),
            ),
        ];
        assert!(matches!(
            crate::validate_route_coverage::validate_route_coverage(&descriptors),
            Err(crate::route_coverage_error::RouteCoverageError::Missing {
                obligation:
                    crate::route_coverage_obligation::RouteCoverageObligation::ReplayValidation,
                ..
            })
        ));
    }

    #[test]
    fn duplicate_route_is_rejected() {
        let descriptor = crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
            route_coverage_metadata(),
            crate::route_access::RouteAccess::Public,
            crate::route_mutation::RouteMutation::ReadOnly,
            crate::route_coverage_evidence::RouteCoverageEvidence::new(&[
                crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
                crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
            ]),
        );
        assert!(matches!(
            crate::validate_route_coverage::validate_route_coverage(&[descriptor, descriptor]),
            Err(crate::route_coverage_error::RouteCoverageError::DuplicateRoute { .. })
        ));
    }

    #[test]
    fn capabilities_require_matching_test_categories() {
        let capabilities = crate::route_test_capabilities::RouteTestCapabilities::new(
            crate::route_database_usage::RouteDatabaseUsage::Database,
            crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody,
            crate::route_response_kind::RouteResponseKind::Streaming,
        );
        assert_eq!(
            bounded_types::bounded_vec::BoundedVec::from(
                crate::missing_required_test_categories::missing_required_test_categories(
                    capabilities,
                    &[
                        crate::route_test_category::RouteTestCategory::FixtureHook,
                        crate::route_test_category::RouteTestCategory::Metadata,
                    ],
                )
            )
            .into_inner(),
            vec![
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ]
        );
    }

    #[test]
    fn routes_without_special_capabilities_require_baseline_categories() {
        let capabilities = crate::route_test_capabilities::RouteTestCapabilities::new(
            crate::route_database_usage::RouteDatabaseUsage::None,
            crate::route_json_body_usage::RouteJsonBodyUsage::None,
            crate::route_response_kind::RouteResponseKind::Buffered,
        );
        assert_eq!(
            bounded_types::bounded_vec::BoundedVec::from(
                crate::required_test_categories::required_test_categories(capabilities)
            )
            .into_inner(),
            vec![
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
            ]
        );
    }
}
