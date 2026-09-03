#![allow(
    unused_crate_dependencies,
    reason = "lint suppression is required here"
)]
#![allow(
    unused_variables,
    reason = "test trait fixtures preserve repository type-based parameter names"
)]

#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
        utoipa::ToSchema,
    )]
    struct TestRequest;
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
        utoipa::ToSchema,
    )]
    struct TestResponse;
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
        utoipa::ToSchema,
    )]
    struct TestErrorResponse;
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    struct TestTransport;
    impl frontend_contract::transport::Transport for TestTransport {
        fn send(
            &self,
            transport_request: frontend_contract::transport_request::TransportRequest,
        ) -> impl Future<
            Output = Result<
                frontend_contract::transport_response::TransportResponse,
                frontend_contract::transport_error::TransportError,
            >,
        > + '_ {
            std::future::ready(Err(
                frontend_contract::transport_error::TransportError::default(),
            ))
        }
    }

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_frontend_contract::TypedRoute,
    )]
    #[typed_route(
        authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
        error_response = TestErrorResponse,
        error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Authentication,
        method = frontend_contract::route_method::RouteMethod::Get,
        mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly,
        obligations = &[
            frontend_contract::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
            frontend_contract::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation,
            frontend_contract::route_coverage_obligation::RouteCoverageObligation::PayloadValidation,
        ],
        openapi_operation_id = constants_str::ROUTE_READ,
        path = constants_str::ROUTE,
        request = TestRequest,
        request_body = frontend_contract::route_request_body::RouteRequestBody::Json,
        response = TestResponse,
        success_status = frontend_contract::success_status::SuccessStatus::Code200,
        transport = frontend_contract::public_transport::PublicTransport,
    )]
    struct TestRoute;

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_frontend_contract::RouteFamily,
    )]
    #[route_family(TestRoute)]
    struct TestRouteFamily;

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        Debug,
        Eq,
        PartialEq,
        proc_macro_frontend_contract::RouteCatalog,
    )]
    #[route_catalog(family = TestCatalogFamily, body_limit = 1024usize)]
    enum TestCatalog {
        #[route_catalog_route(
            contract = frontend_contract::route_contract::RouteContract::new(
                frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
                frontend_contract::route_method::RouteMethod::Get,
                frontend_contract::mutation_kind::MutationKind::ReadOnly,
                frontend_contract::contract_str::ContractStr::from(constants_str::ROUTE),
                frontend_contract::success_status::SuccessStatus::Code200,
            ),
            path = constants_str::ROUTE,
            exclude_from_family,
        )]
        Custom,
        #[route_catalog_route(TestRoute)]
        Read,
    }

    #[test]
    fn test_derive_uses_one_declaration_for_types_and_metadata() {
        let metadata =
            frontend_contract::client_route_metadata::client_route_metadata::<TestRoute>();
        assert_eq!(metadata.method().as_ref(), constants_str::GET);
        assert_eq!(metadata.path().as_ref(), constants_str::ROUTE);
        assert_eq!(
            <TestRoute as frontend_contract::typed_route::TypedRoute>::request_body(),
            frontend_contract::route_request_body::RouteRequestBody::Json
        );
        let _request = frontend_contract::client_request::client_request::<TestRoute>(TestRequest);
        let _response =
            frontend_contract::server_response::server_response::<TestRoute>(TestResponse);
        assert_eq!(
            test_route(),
            frontend_contract::contract_str::ContractStr::from(constants_str::ROUTE)
        );
        assert_eq!(
            size_of_val(&test_client::<TestTransport>),
            constants_usize::ZERO
        );
    }

    #[test]
    fn test_typed_route_registers_request_response_and_problem_schemas() {
        let mut document = utoipa::openapi::OpenApi::default();
        let mut open_api =
            frontend_contract::utoipa_open_api_ref_mut::UtoipaOpenApiRefMut::from(&mut document);
        frontend_contract::register_openapi_route_schemas::register_openapi_route_schemas::<
            TestRoute,
        >(&mut open_api);
        let schemas = &document
            .components
            .expect(constants_str::DIAGNOSTIC_307E6E5F)
            .schemas;
        assert!(schemas.contains_key(constants_str::VALUE_AD93C9A5));
        assert!(schemas.contains_key(constants_str::VALUE_BEF5654C));
        assert!(schemas.contains_key(constants_str::VALUE_7789EA8F));
        assert!(schemas.contains_key(constants_str::VALUE_7FB184D0));
    }

    #[test]
    fn test_typed_route_applies_declared_error_response_schema() {
        let mut operation = utoipa::openapi::path::Operation::default();
        frontend_contract::apply_openapi_error_contract::apply_openapi_error_contract::<TestRoute>(
            &mut operation,
        );
        assert!(operation.responses.responses.values().all(|response_ref| {
            match response_ref {
                utoipa::openapi::RefOr::T(response_value) => response_value
                    .content
                    .contains_key(constants_str::APPLICATION_JSON),
                utoipa::openapi::RefOr::Ref(_reference) => false,
            }
        }));
    }

    #[test]
    fn test_typed_route_applies_declared_json_request_body() {
        let mut operation = utoipa::openapi::path::Operation::default();
        frontend_contract::apply_openapi_request_contract::apply_openapi_request_contract::<
            TestRoute,
        >(&mut operation);
        let request_body = operation
            .request_body
            .expect(constants_str::DIAGNOSTIC_6D9C2D44);
        assert!(matches!(
            request_body.required,
            Some(utoipa::openapi::Required::True)
        ));
        assert!(
            request_body
                .content
                .contains_key(constants_str::APPLICATION_JSON)
        );
    }

    #[test]
    fn test_route_family_generates_valid_coverage_descriptors() {
        let descriptors =
            <TestRouteFamily as frontend_contract::route_family::RouteFamily>::coverage_descriptors(
            );
        assert_eq!(
            <TestRouteFamily as frontend_contract::route_family::RouteFamily>::ROUTE_COUNT,
            constants_usize::ONE
        );
        assert_eq!(descriptors.as_ref().len(), constants_usize::ONE);
        assert_eq!(
            frontend_contract::validate_route_coverage::validate_route_coverage(
                descriptors.as_ref()
            ),
            Ok(())
        );
    }
    #[test]
    fn test_route_family_metadata_comes_from_the_typed_routes() {
        let metadata =
            <TestRouteFamily as frontend_contract::route_family::RouteFamily>::route_metadata();
        assert_eq!(
            metadata,
            frontend_contract::route_metadata_list::RouteMetadataList::from(
                bounded_types::bounded_vec::BoundedVec::from_max_iter([
                    frontend_contract::client_route_metadata::client_route_metadata::<TestRoute>(),
                ]),
            )
        );
    }
    #[test]
    fn test_route_catalog_generates_contract_paths_and_family() {
        assert_eq!(TestCatalog::ALL, [TestCatalog::Custom, TestCatalog::Read]);
        assert_eq!(
            custom_route(),
            frontend_contract::contract_str::ContractStr::from(constants_str::ROUTE)
        );
        assert_eq!(
            size_of_val(&custom_client::<TestTransport>),
            constants_usize::ZERO
        );
        assert_eq!(
            TestCatalog::Read.contract(),
            frontend_contract::client_route_metadata::client_route_metadata::<TestRoute>()
                .contract()
        );
        assert_eq!(
            String::from(TestCatalog::Custom.catalog_path()),
            constants_str::ROUTE
        );
        assert_eq!(
            <TestCatalogFamily as frontend_contract::route_family::RouteFamily>::coverage_descriptors()
                .as_ref()
                .len(),
            constants_usize::ONE
        );
        assert_eq!(
            <TestCatalogFamily as frontend_contract::route_family::RouteFamily>::ROUTE_COUNT,
            constants_usize::ONE
        );
        let schema_contracts =
            <TestCatalogFamily as frontend_contract::route_family::RouteFamily>::schema_contracts();
        assert_eq!(schema_contracts.as_ref().len(), constants_usize::ONE);
        let schema_contract = schema_contracts
            .as_ref()
            .first()
            .expect(constants_str::DIAGNOSTIC_B4E9F1C3);
        assert_eq!(
            schema_contract.metadata(),
            frontend_contract::client_route_metadata::client_route_metadata::<TestRoute>()
        );
        assert!(schema_contract.request_schema().is_some());
        assert!(schema_contract.response_schema().is_some());
    }
}
