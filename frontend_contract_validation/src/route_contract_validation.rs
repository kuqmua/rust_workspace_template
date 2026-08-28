#[path = "http_contract_body.rs"]
mod http_contract_body;
#[path = "http_contract_body_kind.rs"]
mod http_contract_body_kind;
#[path = "http_contract_expectation.rs"]
mod http_contract_expectation;
#[path = "http_contract_mismatch.rs"]
mod http_contract_mismatch;
#[path = "http_contract_observation.rs"]
mod http_contract_observation;
#[path = "http_contract_status.rs"]
mod http_contract_status;
#[path = "route_contract_mismatch.rs"]
mod route_contract_mismatch;
#[path = "route_contract_mismatches.rs"]
mod route_contract_mismatches;
#[path = "run_http_contract_fixture.rs"]
mod run_http_contract_fixture;
#[path = "validate_route_contract_metadata.rs"]
mod validate_route_contract_metadata;
#[path = "validate_typed_route_contract.rs"]
mod validate_typed_route_contract;

pub use http_contract_body::HttpContractBody;
pub use http_contract_body_kind::HttpContractBodyKind;
pub use http_contract_expectation::HttpContractExpectation;
pub use http_contract_mismatch::HttpContractMismatch;
pub use http_contract_observation::HttpContractObservation;
pub use http_contract_status::HttpContractStatus;
pub use route_contract_mismatch::RouteContractMismatch;
pub use route_contract_mismatches::RouteContractMismatches;
pub use run_http_contract_fixture::run_http_contract_fixture;
pub use validate_route_contract_metadata::validate_route_contract_metadata;
pub use validate_typed_route_contract::validate_typed_route_contract;

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct ReadRoute;
    impl frontend_contract::domain_types::TypedRoute for ReadRoute {
        type Request = ();
        type Response = ();
        type Transport = frontend_contract::domain_types::PublicTransport;

        fn metadata() -> frontend_contract::domain_types::RouteMetadata {
            metadata(
                frontend_contract::domain_types::RouteMethod::Get,
                constants_str::ROUTE_READ,
                constants_str::ROUTE,
            )
        }
    }

    fn metadata(
        method: frontend_contract::domain_types::RouteMethod,
        operation_id: &'static str,
        path: &'static str,
    ) -> frontend_contract::domain_types::RouteMetadata {
        frontend_contract::domain_types::RouteMetadata::new(
            method,
            operation_id.into(),
            path.into(),
        )
    }

    #[test]
    fn equal_metadata_satisfies_contract() {
        let metadata = metadata(
            frontend_contract::domain_types::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        assert_eq!(
            super::validate_route_contract_metadata(metadata, metadata),
            Ok(())
        );
    }

    #[test]
    fn typed_route_is_the_contract_source_of_truth() {
        assert_eq!(
            super::validate_typed_route_contract::<ReadRoute>(metadata(
                frontend_contract::domain_types::RouteMethod::Get,
                constants_str::ROUTE_READ,
                constants_str::ROUTE
            )),
            Ok(())
        );
    }

    #[test]
    fn http_fixture_checks_status_and_json_body() {
        let metadata = metadata(
            frontend_contract::domain_types::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        let result = futures::executor::block_on(super::run_http_contract_fixture(
            super::HttpContractExpectation::new(
                metadata,
                super::HttpContractStatus::try_from(200u16).expect(
                    "a76c9e6b http_fixture_checks_status_and_json_body invariant must hold",
                ),
                super::HttpContractBodyKind::Json,
            ),
            async |observed_metadata| {
                super::HttpContractObservation::new(
                    observed_metadata,
                    super::HttpContractStatus::try_from(200u16).expect(
                        "d0abdccc http_fixture_checks_status_and_json_body invariant must hold",
                    ),
                    super::HttpContractBody::try_from(br#"{"ok":true}"#.to_vec()).expect(
                        "08bddb5e http_fixture_checks_status_and_json_body invariant must hold",
                    ),
                )
            },
        ));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn every_metadata_difference_is_reported() {
        let expected = metadata(
            frontend_contract::domain_types::RouteMethod::Get,
            constants_str::ROUTE_READ,
            constants_str::ROUTE,
        );
        let observed = metadata(
            frontend_contract::domain_types::RouteMethod::Post,
            constants_str::ADMIN_ALT,
            constants_str::NOT_AN_API_ROUTE,
        );
        let mismatches = super::validate_route_contract_metadata(expected, observed)
            .expect_err(constants_str::VALUE_5067F83C);
        assert_eq!(mismatches.as_ref().len(), 3usize);
        assert!(matches!(
            mismatches.as_ref().first(),
            Some(super::RouteContractMismatch::Method { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(constants_usize::ONE),
            Some(super::RouteContractMismatch::OpenApiOperationId { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(2usize),
            Some(super::RouteContractMismatch::Path { .. })
        ));
    }
}
