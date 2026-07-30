#![allow(unused_crate_dependencies)] // integration test links the package dependency set while exercising the re-exported derive macro

#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
    struct TestRequest;
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
    struct TestResponse;
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
    struct TestErrorResponse;

    #[derive(frontend_contract::TypedRoute)]
    #[typed_route(
        authentication = frontend_contract::AuthenticationRequirement::Public,
        error_response = TestErrorResponse,
        error_policy = frontend_contract::RouteErrorPolicy::Authentication,
        method = frontend_contract::RouteMethod::Get,
        mutation = frontend_contract::RouteMutation::ReadOnly,
        obligations = &[
            frontend_contract::RouteCoverageObligation::IntegrationFixture,
            frontend_contract::RouteCoverageObligation::OpenApiOperation,
            frontend_contract::RouteCoverageObligation::PayloadValidation,
        ],
        openapi_operation_id = str_constants::ROUTE_READ,
        path = str_constants::ROUTE,
        request = TestRequest,
        request_body = frontend_contract::RouteRequestBody::Json,
        response = TestResponse,
        success_status = frontend_contract::SuccessStatus::Code200,
        transport = frontend_contract::PublicTransport,
    )]
    struct TestRoute;

    #[derive(frontend_contract::RouteFamily)]
    #[route_family(TestRoute)]
    struct TestRouteFamily;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, frontend_contract::RouteCatalog)]
    #[route_catalog(family = TestCatalogFamily, body_limit = 1024usize)]
    enum TestCatalog {
        #[route_catalog_route(
            contract = frontend_contract::RouteContract::new(
                frontend_contract::AuthenticationRequirement::Public,
                frontend_contract::HttpMethod::Get,
                frontend_contract::MutationKind::ReadOnly,
                frontend_contract::ContractStr::from(str_constants::ROUTE),
                frontend_contract::SuccessStatus::Code200,
            ),
            path = str_constants::ROUTE,
            exclude_from_family,
        )]
        Custom,
        #[route_catalog_route(TestRoute)]
        Read,
    }

    #[test]
    fn derive_uses_one_declaration_for_types_and_metadata() {
        let metadata = frontend_contract::client_route_metadata::<TestRoute>();
        assert_eq!(metadata.method().as_ref(), str_constants::GET);
        assert_eq!(metadata.path().as_ref(), str_constants::ROUTE);
        assert_eq!(
            <TestRoute as frontend_contract::TypedRoute>::request_body(),
            frontend_contract::RouteRequestBody::Json
        );
        let _request = frontend_contract::client_request::<TestRoute>(TestRequest);
        let _response = frontend_contract::server_response::<TestRoute>(TestResponse);
    }

    #[test]
    fn typed_route_registers_request_response_and_problem_schemas() {
        let mut document = utoipa::openapi::OpenApi::default();
        let mut open_api = frontend_contract::UtoipaOpenApiRefMut::from(&mut document);
        frontend_contract::register_openapi_route_schemas::<TestRoute>(&mut open_api);
        let schemas = &document.components.expect("307e6e5f").schemas;
        assert!(schemas.contains_key("TestRequest"));
        assert!(schemas.contains_key("TestResponse"));
        assert!(schemas.contains_key("TestErrorResponse"));
        assert!(schemas.contains_key("ApiProblem"));
    }

    #[test]
    fn typed_route_applies_declared_error_response_schema() {
        let mut operation = utoipa::openapi::path::Operation::default();
        frontend_contract::apply_openapi_error_contract::<TestRoute>(&mut operation);
        assert!(operation.responses.responses.values().all(|response_ref| {
            match response_ref {
                utoipa::openapi::RefOr::T(response_value) => response_value
                    .content
                    .contains_key(str_constants::APPLICATION_JSON),
                utoipa::openapi::RefOr::Ref(_reference) => false,
            }
        }));
    }

    #[test]
    fn typed_route_applies_declared_json_request_body() {
        let mut operation = utoipa::openapi::path::Operation::default();
        frontend_contract::apply_openapi_request_contract::<TestRoute>(&mut operation);
        let request_body = operation.request_body.expect("6d9c2d44");
        assert!(matches!(
            request_body.required,
            Some(utoipa::openapi::Required::True)
        ));
        assert!(
            request_body
                .content
                .contains_key(str_constants::APPLICATION_JSON)
        );
    }

    #[test]
    fn route_family_generates_valid_coverage_descriptors() {
        let descriptors =
            <TestRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
        assert_eq!(
            <TestRouteFamily as frontend_contract::RouteFamily>::ROUTE_COUNT,
            1usize
        );
        assert_eq!(descriptors.as_ref().len(), 1usize);
        assert_eq!(
            frontend_contract::validate_route_coverage(descriptors.as_ref()),
            Ok(())
        );
    }
    #[test]
    fn route_family_metadata_comes_from_the_typed_routes() {
        let metadata = <TestRouteFamily as frontend_contract::RouteFamily>::route_metadata();
        assert_eq!(
            metadata,
            frontend_contract::RouteMetadataList::from(bounded_types::BoundedVec::from_max_iter([
                frontend_contract::client_route_metadata::<TestRoute>(),
            ]),)
        );
    }
    #[test]
    fn route_catalog_generates_contract_paths_and_family() {
        assert_eq!(TestCatalog::ALL, [TestCatalog::Custom, TestCatalog::Read]);
        assert_eq!(
            TestCatalog::Read.contract(),
            frontend_contract::client_route_metadata::<TestRoute>().contract()
        );
        assert_eq!(
            String::from(TestCatalog::Custom.catalog_path()),
            str_constants::ROUTE
        );
        assert_eq!(
            <TestCatalogFamily as frontend_contract::RouteFamily>::coverage_descriptors()
                .as_ref()
                .len(),
            1usize
        );
        assert_eq!(
            <TestCatalogFamily as frontend_contract::RouteFamily>::ROUTE_COUNT,
            1usize
        );
        let schema_contracts =
            <TestCatalogFamily as frontend_contract::RouteFamily>::schema_contracts();
        assert_eq!(schema_contracts.as_ref().len(), 1usize);
        let schema_contract = schema_contracts.as_ref().first().expect("b4e9f1c3");
        assert_eq!(
            schema_contract.metadata(),
            frontend_contract::client_route_metadata::<TestRoute>()
        );
        assert!(schema_contract.request_schema().is_some());
        assert!(schema_contract.response_schema().is_some());
    }
}
