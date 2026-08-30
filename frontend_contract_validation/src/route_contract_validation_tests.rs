#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct ReadRoute;
    impl frontend_contract::typed_route::TypedRoute for ReadRoute {
        type Request = ();
        type Response = ();
        type Transport = frontend_contract::public_transport::PublicTransport;

        fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
            route_validation_metadata(
                frontend_contract::route_method::RouteMethod::Get,
                constants_str::catalog::ROUTE_READ,
                constants_str::catalog::ROUTE,
            )
        }
    }

    fn route_validation_metadata(
        method: frontend_contract::route_method::RouteMethod,
        operation_id: &'static str,
        path: &'static str,
    ) -> frontend_contract::route_metadata::RouteMetadata {
        frontend_contract::route_metadata::RouteMetadata::new(
            method,
            operation_id.into(),
            path.into(),
        )
    }

    #[test]
    fn equal_metadata_satisfies_contract() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::catalog::ROUTE_READ,
            constants_str::catalog::ROUTE,
        );
        assert_eq!(
            crate::validate_route_contract_metadata::validate_route_contract_metadata(
                metadata, metadata
            ),
            Ok(())
        );
    }

    #[test]
    fn typed_route_is_the_contract_source_of_truth() {
        assert_eq!(
            crate::validate_typed_route_contract::validate_typed_route_contract::<ReadRoute>(
                route_validation_metadata(
                    frontend_contract::route_method::RouteMethod::Get,
                    constants_str::catalog::ROUTE_READ,
                    constants_str::catalog::ROUTE
                )
            ),
            Ok(())
        );
    }

    #[test]
    fn http_fixture_checks_status_and_json_body() {
        let metadata = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::catalog::ROUTE_READ,
            constants_str::catalog::ROUTE,
        );
        let expectation = crate::http_contract_expectation::HttpContractExpectation::new(
            metadata,
            crate::http_contract_status::HttpContractStatus::try_from(200u16)
                .expect("a76c9e6b http_fixture_checks_status_and_json_body invariant must hold"),
            crate::http_contract_body_kind::HttpContractBodyKind::Json,
        );
        let (_, _, body_kind) = expectation.parts();
        assert_eq!(
            body_kind,
            crate::http_contract_body_kind::HttpContractBodyKind::Json
        );
        let result = futures::executor::block_on(
            crate::run_http_contract_fixture::run_http_contract_fixture(
                expectation,
                async |observed_metadata| {
                    let observation = crate::http_contract_observation::HttpContractObservation::new(
                        observed_metadata,
                        crate::http_contract_status::HttpContractStatus::try_from(200u16).expect(
                            "d0abdccc http_fixture_checks_status_and_json_body invariant must hold",
                        ),
                        crate::http_contract_body::HttpContractBody::try_from(
                            br#"{"ok":true}"#.to_vec(),
                        )
                        .expect(
                            "08bddb5e http_fixture_checks_status_and_json_body invariant must hold",
                        ),
                    );
                    let (body, _, _) = observation.parts();
                    assert!(!body.is_empty());
                    observation
                },
            ),
        );
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn every_metadata_difference_is_reported() {
        let expected = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Get,
            constants_str::catalog::ROUTE_READ,
            constants_str::catalog::ROUTE,
        );
        let observed = route_validation_metadata(
            frontend_contract::route_method::RouteMethod::Post,
            constants_str::catalog::ADMIN_ALT,
            constants_str::catalog::NOT_AN_API_ROUTE,
        );
        let mismatches = crate::validate_route_contract_metadata::validate_route_contract_metadata(
            expected, observed,
        )
        .expect_err(constants_str::catalog::VALUE_5067F83C);
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
}
