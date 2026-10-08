#[cfg(test)]
mod tests {
    fn route_coverage_metadata() -> crate::route_metadata::RouteMetadata {
        crate::route_metadata::RouteMetadata::new(
            crate::route_method::RouteMethod::Post,
            constants_str::ROUTE_READ.into(),
            constants_str::ROUTE.into(),
        )
    }

    #[test]
    fn test_complete_mutating_authenticated_route_is_covered() {
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
    fn test_mutating_route_requires_replay_validation() {
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
    fn test_duplicate_route_is_rejected() {
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
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[descriptor, descriptor]),
            Err(
                crate::route_coverage_error::RouteCoverageError::DuplicateRoute {
                    metadata: *descriptor.get_metadata()
                }
            )
        );
    }

    #[test]
    fn test_same_method_and_path_with_different_operation_ids_is_duplicate() {
        let descriptor = |route_metadata: crate::route_metadata::RouteMetadata| {
            crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
                route_metadata,
                crate::route_access::RouteAccess::Public,
                crate::route_mutation::RouteMutation::ReadOnly,
                crate::route_coverage_evidence::RouteCoverageEvidence::new(
                    crate::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
                ),
            )
        };
        let first = descriptor(route_coverage_metadata());
        let second = descriptor(crate::route_metadata::RouteMetadata::new(
            crate::route_method::RouteMethod::Post,
            constants_str::ROUTE.into(),
            constants_str::ROUTE.into(),
        ));
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[first, second]),
            Err(
                crate::route_coverage_error::RouteCoverageError::DuplicateRoute {
                    metadata: *second.get_metadata()
                }
            )
        );
        let different_method = descriptor(crate::route_metadata::RouteMetadata::new(
            crate::route_method::RouteMethod::Get,
            constants_str::ROUTE.into(),
            constants_str::ROUTE.into(),
        ));
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[first, different_method]),
            Ok(())
        );
    }

    #[test]
    fn test_route_coverage_preserves_descriptor_order_and_duplicate_error_precedence() {
        let descriptor = |route_metadata: crate::route_metadata::RouteMetadata, route_coverage_evidence: crate::route_coverage_evidence::RouteCoverageEvidence| {
            crate::route_coverage_descriptor::RouteCoverageDescriptor::new(route_metadata, crate::route_access::RouteAccess::Public, crate::route_mutation::RouteMutation::ReadOnly, route_coverage_evidence)
        };
        let first_metadata = route_coverage_metadata();
        let distinct_metadata = crate::route_metadata::RouteMetadata::new(
            crate::route_method::RouteMethod::Post,
            constants_str::X.into(),
            constants_str::X.into(),
        );
        let duplicate_metadata = crate::route_metadata::RouteMetadata::new(
            crate::route_method::RouteMethod::Post,
            constants_str::FIELD.into(),
            constants_str::ROUTE.into(),
        );
        let complete = crate::route_coverage_evidence::RouteCoverageEvidence::new(
            crate::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
        );
        let empty = crate::route_coverage_evidence::RouteCoverageEvidence::new(&[]);
        let first = descriptor(first_metadata, complete);
        let distinct = descriptor(distinct_metadata, complete);
        let duplicate = descriptor(duplicate_metadata, empty);
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[]),
            Ok(())
        );
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[first, distinct]),
            Ok(())
        );
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[first, distinct, duplicate]),
            Err(
                crate::route_coverage_error::RouteCoverageError::DuplicateRoute {
                    metadata: duplicate_metadata
                }
            )
        );
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[
                descriptor(first_metadata, empty),
                distinct,
                duplicate
            ]),
            Err(crate::route_coverage_error::RouteCoverageError::Missing {
                metadata: first_metadata,
                obligation:
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture
            })
        );
        assert_eq!(
            crate::validate_route_coverage::validate_route_coverage(&[
                first,
                descriptor(distinct_metadata, empty),
                duplicate
            ]),
            Err(crate::route_coverage_error::RouteCoverageError::Missing {
                metadata: distinct_metadata,
                obligation:
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture
            })
        );
    }

    #[test]
    fn test_capabilities_require_matching_test_categories() {
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
            [
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ]
        );
    }

    #[test]
    fn test_routes_without_special_capabilities_require_baseline_categories() {
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
            [
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
            ]
        );
    }
    #[test]
    fn test_route_coverage_reports_first_missing_baseline_obligation_with_metadata() {
        let metadata = route_coverage_metadata();
        let cases = [
            (
                crate::route_coverage_evidence::RouteCoverageEvidence::new(&[]),
                crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
            ),
            (
                crate::route_coverage_evidence::RouteCoverageEvidence::new(&[
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                ]),
                crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
            ),
            (
                crate::route_coverage_evidence::RouteCoverageEvidence::new(&[
                    crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
                    crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
                ]),
                crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
            ),
        ];
        assert!(cases.into_iter().all(|(evidence, obligation)| {
            let descriptor = crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
                metadata,
                crate::route_access::RouteAccess::Authenticated,
                crate::route_mutation::RouteMutation::Mutating,
                evidence,
            );
            crate::validate_route_coverage::validate_route_coverage(&[descriptor])
                == Err(crate::route_coverage_error::RouteCoverageError::Missing {
                    metadata,
                    obligation,
                })
        }));
    }
    #[test]
    fn test_route_coverage_security_precedes_replay_for_authenticated_mutations() {
        let metadata = route_coverage_metadata();
        let cases = [
            (
                crate::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
                crate::route_coverage_obligation::RouteCoverageObligation::SecurityValidation,
            ),
            (
                crate::route_coverage_obligation::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
                crate::route_coverage_obligation::RouteCoverageObligation::SecurityValidation,
            ),
            (
                crate::route_coverage_obligation::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS,
                crate::route_coverage_obligation::RouteCoverageObligation::ReplayValidation,
            ),
        ];
        assert!(cases.into_iter().all(|(obligations, obligation)| {
            let descriptor = crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
                metadata,
                crate::route_access::RouteAccess::Authenticated,
                crate::route_mutation::RouteMutation::Mutating,
                crate::route_coverage_evidence::RouteCoverageEvidence::new(obligations),
            );
            crate::validate_route_coverage::validate_route_coverage(&[descriptor])
                == Err(crate::route_coverage_error::RouteCoverageError::Missing {
                    metadata,
                    obligation,
                })
        }));
    }

    #[test]
    fn test_missing_route_test_category_matrix_ignores_evidence_order_and_duplicates() {
        let capabilities = crate::route_test_capabilities::RouteTestCapabilities::new(
            crate::route_database_usage::RouteDatabaseUsage::Database,
            crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody,
            crate::route_response_kind::RouteResponseKind::Streaming,
        );
        let categories = [
            (
                1u8,
                crate::route_test_category::RouteTestCategory::FixtureHook,
            ),
            (2u8, crate::route_test_category::RouteTestCategory::Metadata),
            (
                4u8,
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
            ),
            (
                8u8,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
            ),
            (
                16u8,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ),
        ];
        (0u8..32u8).for_each(|mask| {
            let mut available = categories
                .iter()
                .rev()
                .filter(|(bit, _category)| mask & bit != 0u8)
                .map(|(_bit, category)| *category)
                .collect::<Vec<_>>();
            available.extend_from_within(..);
            let expected = categories
                .iter()
                .filter(|(bit, _category)| mask & bit == 0u8)
                .map(|(_bit, category)| *category)
                .collect::<Vec<_>>();
            let missing = crate::missing_required_test_categories::missing_required_test_categories(
                capabilities,
                &available,
            );
            assert_eq!(missing.as_ref(), expected.as_slice());
        });
    }
}
