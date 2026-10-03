#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct ReadRoute;
    impl frontend_contract::typed_route::TypedRoute for ReadRoute {
        type Request = ();
        type Response = ();
        type Transport = frontend_contract::public_transport::PublicTransport;

        fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
            route_validation_metadata(
                frontend_contract::route_method::RouteMethod::Get,
                constants_str::ROUTE_READ,
                constants_str::ROUTE,
            )
        }
    }

    fn route_validation_metadata(
        route_method: frontend_contract::route_method::RouteMethod,
        operation_id: &'static str,
        path: &'static str,
    ) -> frontend_contract::route_metadata::RouteMetadata {
        frontend_contract::route_metadata::RouteMetadata::new(
            route_method,
            operation_id.into(),
            path.into(),
        )
    }

    #[test]
    fn test_equal_metadata_satisfies_contract() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        assert_eq!(
            crate::validate_route_contract_metadata::validate_route_contract_metadata(
                metadata, metadata
            ),
            Ok(())
        );
    }

    #[test]
    fn test_typed_route_is_the_contract_source_of_truth() {
        assert_eq!(
            crate::validate_typed_route_contract::validate_typed_route_contract::<ReadRoute>(
                route_validation_metadata(
                    frontend_contract::route_method::RouteMethod::Get,
                    constants_str::ROUTE_READ,
                    constants_str::ROUTE
                )
            ),
            Ok(())
        );
    }

    #[test]
    fn test_http_fixture_checks_status_and_json_body() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        let expectation = crate::http_contract_expectation::HttpContractExpectation::new(
            metadata,
            crate::http_contract_status::HttpContractStatus::try_from(200u16)
                .expect(constants_str::DIAGNOSTIC_A76C9E6B),
            crate::http_contract_body_kind::HttpContractBodyKind::Json,
        );
        assert_eq!(
            expectation.body_kind(),
            crate::http_contract_body_kind::HttpContractBodyKind::Json
        );
        let result = futures::executor::block_on(
            crate::run_http_contract_fixture::run_http_contract_fixture(
                expectation,
                async |observed_metadata| {
                    let observation =
                        crate::http_contract_observation::HttpContractObservation::new(
                            observed_metadata,
                            crate::http_contract_status::HttpContractStatus::try_from(200u16)
                                .expect(constants_str::DIAGNOSTIC_D0ABDCCC),
                            crate::http_contract_body::HttpContractBody::try_from(
                                br#"{"ok":true}"#.to_vec(),
                            )
                            .expect(constants_str::DIAGNOSTIC_08BDDB5E),
                        );
                    assert!(!observation.body().is_empty());
                    observation
                },
            ),
        );
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_every_metadata_difference_is_reported() {
        let expected = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        let observed = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Post,
            constants_str::ADMIN_ALT,
            constants_str::NOT_AN_API_ROUTE,
        );
        let mismatches = crate::validate_route_contract_metadata::validate_route_contract_metadata(
            expected, observed,
        )
        .expect_err(constants_str::VALUE_5067F83C);
        assert_eq!(mismatches.as_ref().len(), 3usize);
        assert!(matches!(
            mismatches.as_ref().first(),
            Some(crate::route_contract_mismatch::RouteContractMismatch::Method { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(constants_usize::ONE),
            Some(crate::route_contract_mismatch::RouteContractMismatch::OpenApiOperationId { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(2usize),
            Some(crate::route_contract_mismatch::RouteContractMismatch::Path { .. })
        ));
    }
    #[test]
    fn test_http_fixture_checks_empty_and_json_body_failure_categories() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        assert!(crate::http_contract_status::HttpContractStatus::try_from(200u16).is_ok_and(|status| {
            [
                (crate::http_contract_body_kind::HttpContractBodyKind::Empty, constants_str::EMPTY, Ok(())),
                (crate::http_contract_body_kind::HttpContractBodyKind::Empty, constants_str::X, Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedEmpty)),
                (crate::http_contract_body_kind::HttpContractBodyKind::Json, constants_str::EMPTY, Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedJson)),
                (crate::http_contract_body_kind::HttpContractBodyKind::Json, constants_str::X, Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedJson)),
                (crate::http_contract_body_kind::HttpContractBodyKind::Json, constants_str::VALUE_1, Ok(())),
                (crate::http_contract_body_kind::HttpContractBodyKind::Json, constants_str::VALUE_4F53CDA1, Ok(())),
            ].into_iter().all(|(body_kind, text, expected)| {
                crate::http_contract_body::HttpContractBody::try_from(text.as_bytes().to_vec()).is_ok_and(|body| {
                    let expectation = crate::http_contract_expectation::HttpContractExpectation::new(metadata, status, body_kind);
                    let calls = std::cell::Cell::new(0usize);
                    let observed = futures::executor::block_on(crate::run_http_contract_fixture::run_http_contract_fixture(expectation, |sent_metadata| {
                        assert_eq!(sent_metadata, metadata);
                        calls.set(calls.get() + constants_usize::ONE);
                        std::future::ready(crate::http_contract_observation::HttpContractObservation::new(sent_metadata, status, body))
                    }));
                    observed == expected && calls.get() == constants_usize::ONE
                })
            })
        }));
    }

    #[test]
    fn test_http_fixture_reports_status_before_body_mismatch() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        assert!(crate::http_contract_status::HttpContractStatus::try_from(200u16).is_ok_and(|expected_status| {
            crate::http_contract_status::HttpContractStatus::try_from(401u16).is_ok_and(|observed_status| {
                crate::http_contract_body::HttpContractBody::try_from(constants_str::X.as_bytes().to_vec()).is_ok_and(|body| {
                    let expectation = crate::http_contract_expectation::HttpContractExpectation::new(metadata, expected_status, crate::http_contract_body_kind::HttpContractBodyKind::Json);
                    let actual = futures::executor::block_on(crate::run_http_contract_fixture::run_http_contract_fixture(expectation, |sent_metadata| std::future::ready(crate::http_contract_observation::HttpContractObservation::new(sent_metadata, observed_status, body))));
                    actual == Err(crate::http_contract_mismatch::HttpContractMismatch::Status { expected: expected_status, observed: observed_status })
                })
            })
        }));
    }
    #[test]
    fn test_http_fixture_reports_metadata_before_status_and_body_mismatches() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        let wrong_metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Post,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        assert!(crate::http_contract_status::HttpContractStatus::try_from(200u16).is_ok_and(|expected_status| {
            crate::http_contract_status::HttpContractStatus::try_from(401u16).is_ok_and(|observed_status| {
                crate::http_contract_body::HttpContractBody::try_from(constants_str::X.as_bytes().to_vec()).is_ok_and(|body| {
                    let expectation = crate::http_contract_expectation::HttpContractExpectation::new(metadata, expected_status, crate::http_contract_body_kind::HttpContractBodyKind::Json);
                    let actual = futures::executor::block_on(crate::run_http_contract_fixture::run_http_contract_fixture(expectation, |sent_metadata| {
                        assert_eq!(sent_metadata, metadata);
                        std::future::ready(crate::http_contract_observation::HttpContractObservation::new(wrong_metadata, observed_status, body))
                    }));
                    actual.is_err_and(|error| matches!(error, crate::http_contract_mismatch::HttpContractMismatch::Metadata(mismatches) if mismatches.as_ref() == [crate::route_contract_mismatch::RouteContractMismatch::Method { expected: metadata.method(), observed: wrong_metadata.method() }]))
                })
            })
        }));
    }
}
