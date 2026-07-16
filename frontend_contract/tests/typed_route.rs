#![allow(unused_crate_dependencies)] // integration test links the package dependency set while exercising the re-exported derive macro

#[cfg(test)]
mod tests {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    struct TestRequest;
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    struct TestResponse;

    #[derive(frontend_contract::TypedRoute)]
    #[typed_route(
        access = frontend_contract::RouteAccess::Public,
        method = str_constants::GET,
        mutation = frontend_contract::RouteMutation::ReadOnly,
        obligations = &[
            frontend_contract::RouteCoverageObligation::IntegrationFixture,
            frontend_contract::RouteCoverageObligation::OpenApiOperation,
            frontend_contract::RouteCoverageObligation::PayloadValidation,
        ],
        openapi_operation_id = str_constants::ROUTE_READ,
        path = str_constants::ROUTE,
        request = TestRequest,
        response = TestResponse,
        transport = frontend_contract::PublicTransport,
    )]
    struct TestRoute;

    #[derive(frontend_contract::RouteFamily)]
    #[route_family(TestRoute)]
    struct TestRouteFamily;

    #[test]
    fn derive_uses_one_declaration_for_types_and_metadata() {
        let metadata = frontend_contract::client_route_metadata::<TestRoute>();
        assert_eq!(metadata.method().as_ref(), str_constants::GET);
        assert_eq!(metadata.path().as_ref(), str_constants::ROUTE);
        let _request = frontend_contract::client_request::<TestRoute>(TestRequest);
        let _response = frontend_contract::server_response::<TestRoute>(TestResponse);
    }

    #[test]
    fn route_family_generates_valid_coverage_descriptors() {
        let descriptors =
            <TestRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
        assert_eq!(descriptors.len(), 1usize);
        assert_eq!(
            frontend_contract::validate_route_coverage(&descriptors),
            Ok(())
        );
    }
}
