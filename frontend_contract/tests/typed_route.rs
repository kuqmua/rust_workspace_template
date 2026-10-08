#![allow(
    unused_crate_dependencies,
    reason = "typed route requires this localized allowance for generated or framework-constrained code verified by focused tests"
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
        proc_macro_frontend_contract_derive_typed_route::TypedRoute,
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
        proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
        proc_macro_newtype_display::Display,
        utoipa::ToSchema,
    )]
    #[bounded_string(max = 8192usize)]
    struct TestLongRouteParameter(
        bounded_types::bounded_string::BoundedString<0usize, 8192usize, false>,
    );

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_frontend_contract_derive_typed_route::TypedRoute,
    )]
    #[typed_route(
        authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
        error_response = TestErrorResponse,
        error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Authentication,
        method = frontend_contract::route_method::RouteMethod::Get,
        mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly,
        obligations = &[
            frontend_contract::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture,
        ],
        openapi_operation_id = constants_str::ROUTE_READ,
        path = "/items/{item_id}",
        path_parameter = TestLongRouteParameter,
        request = TestRequest,
        response = TestResponse,
        success_status = frontend_contract::success_status::SuccessStatus::Code200,
        transport = frontend_contract::public_transport::PublicTransport,
    )]
    struct TestLongParameterizedRoute;

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_frontend_contract_derive_route_family::RouteFamily,
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
        proc_macro_frontend_contract_derive_route_catalog::RouteCatalog,
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

    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_frontend_contract_derive_route_catalog::RouteCatalog,
    )]
    #[route_catalog(family = TestParameterizedCatalogFamily, body_limit = 1024usize)]
    enum TestParameterizedCatalog {
        #[route_catalog_route(TestLongParameterizedRoute)]
        Long(TestLongRouteParameter),
    }

    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct TestDefaultHooksRoute;

    impl frontend_contract::typed_route::TypedRoute for TestDefaultHooksRoute {
        type Request = TestRequest;
        type Response = TestResponse;
        type Transport = frontend_contract::public_transport::PublicTransport;

        fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
            <TestRoute as frontend_contract::typed_route::TypedRoute>::metadata()
        }
    }

    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct ErrorSchemaAbsentContractTestRoute;
    impl frontend_contract::typed_route::TypedRoute for ErrorSchemaAbsentContractTestRoute {
        type Request = TestRequest;
        type Response = TestResponse;
        type Transport = frontend_contract::public_transport::PublicTransport;
        fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
            <TestRoute as frontend_contract::typed_route::TypedRoute>::metadata()
        }
        fn openapi_error_response_schema(
            route_error_status: frontend_contract::route_error_status::RouteErrorStatus,
        ) -> Option<frontend_contract::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema>
        {
            None
        }
    }

    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct CreatedContractTestRoute;
    impl frontend_contract::typed_route::TypedRoute for CreatedContractTestRoute {
        type Request = TestRequest;
        type Response = TestResponse;
        type Transport = frontend_contract::public_transport::PublicTransport;
        fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
            success_contract_test_metadata(
                frontend_contract::success_status::SuccessStatus::Code201,
            )
        }
        fn openapi_response_schema()
        -> Option<frontend_contract::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema>
        {
            <TestRoute as frontend_contract::typed_route::TypedRoute>::openapi_response_schema()
        }
    }

    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct NoContentContractTestRoute;
    impl frontend_contract::typed_route::TypedRoute for NoContentContractTestRoute {
        type Request = TestRequest;
        type Response = TestResponse;
        type Transport = frontend_contract::public_transport::PublicTransport;
        fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
            success_contract_test_metadata(
                frontend_contract::success_status::SuccessStatus::Code204,
            )
        }
        fn openapi_response_schema()
        -> Option<frontend_contract::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema>
        {
            <TestRoute as frontend_contract::typed_route::TypedRoute>::openapi_response_schema()
        }
    }

    #[test]
    fn test_schema_contract_preserves_absent_hooks_and_metadata() {
        let route_schema_contract =
            frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<
                TestDefaultHooksRoute,
            >();
        assert_eq!(
            route_schema_contract.metadata(),
            <TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::metadata()
        );
        assert!(route_schema_contract.request_schema().is_none());
        assert!(route_schema_contract.response_schema().is_none());
    }

    #[test]
    fn test_schema_contract_forwards_exact_schemas_and_independent_optional_hooks() {
        let cases = [
            (frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<TestRoute>(), <TestRoute as frontend_contract::typed_route::TypedRoute>::metadata(), Some(<TestRequest as utoipa::PartialSchema>::schema())),
            (frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<CreatedContractTestRoute>(), <CreatedContractTestRoute as frontend_contract::typed_route::TypedRoute>::metadata(), None),
            (frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<NoContentContractTestRoute>(), <NoContentContractTestRoute as frontend_contract::typed_route::TypedRoute>::metadata(), None),
        ];
        assert!(cases.into_iter().all(
            |(route_schema_contract, route_metadata, request_schema)| {
                route_schema_contract.metadata() == route_metadata
                    && route_schema_contract
                        .request_schema()
                        .cloned()
                        .map(utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from)
                        == request_schema
                    && route_schema_contract
                        .response_schema()
                        .cloned()
                        .map(utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from)
                        == Some(<TestResponse as utoipa::PartialSchema>::schema())
                    && route_schema_contract
                        .response_schema()
                        .zip(route_schema_contract.response_schema())
                        .is_some_and(|(first, second)| std::ptr::eq(first, second))
            }
        ));
    }

    #[test]
    fn test_default_typed_route_schema_hooks_and_error_statuses() {
        assert!(<TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::openapi_request_schema().is_none());
        assert!(<TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::openapi_request_body_schema().is_none());
        assert!(<TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::openapi_response_schema().is_none());
        assert!(<TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::openapi_path_parameter().is_none());
        assert_eq!(
            <TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::request_body(),
            frontend_contract::route_request_body::RouteRequestBody::Absent,
        );
        assert!([
            frontend_contract::route_error_status::RouteErrorStatus::Authentication,
            frontend_contract::route_error_status::RouteErrorStatus::Authorization,
            frontend_contract::route_error_status::RouteErrorStatus::Conflict,
            frontend_contract::route_error_status::RouteErrorStatus::Internal,
            frontend_contract::route_error_status::RouteErrorStatus::MethodNotAllowed,
            frontend_contract::route_error_status::RouteErrorStatus::PayloadTooLarge,
            frontend_contract::route_error_status::RouteErrorStatus::RateLimited,
            frontend_contract::route_error_status::RouteErrorStatus::ServiceUnavailable,
            frontend_contract::route_error_status::RouteErrorStatus::Validation,
        ].into_iter().all(|status| {
            <TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::openapi_error_response_schema(status)
                .is_some_and(|schema| {
                    utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from(schema)
                        == <frontend_contract::api_problem::ApiProblem as utoipa::PartialSchema>::schema()
                })
        }));
    }

    #[test]
    fn test_default_typed_route_schema_registration_preserves_components() {
        let mut components = utoipa::openapi::schema::Components::default();
        let _previous = components.schemas.insert(
            constants_str::ROUTE.to_owned(),
            <TestRequest as utoipa::PartialSchema>::schema(),
        );
        let expected = components.clone();
        <TestDefaultHooksRoute as frontend_contract::typed_route::TypedRoute>::register_openapi_schemas(
            &mut frontend_contract::utoipa_open_api_components_ref_mut::UtoipaOpenApiComponentsRefMut::from(&mut components),
        );
        assert!(components == expected);
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
            frontend_contract::typed_route_path::typed_route_path::<TestRoute>(),
            frontend_contract::contract_str::ContractStr::from(constants_str::ROUTE)
        );
        assert_eq!(
            size_of_val(&test_client::<TestTransport>),
            constants_usize::ZERO
        );
    }

    #[test]
    fn test_oversized_generated_parameterized_path_returns_client_encode_error() {
        let parameter_result = TestLongRouteParameter::try_from(constants_str::X.repeat(8192usize));
        assert!(parameter_result.is_ok());
        if let Ok(parameter) = parameter_result {
            assert!(test_long_parameterized_route(&parameter).is_err_and(|error| {
                error
                    == frontend_contract::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError::TooLong
            }));
            assert_eq!(
                frontend_contract::typed_parameterized_route_path::typed_parameterized_route_path::<TestLongParameterizedRoute>(&parameter),
                Err(frontend_contract::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError::TooLong)
            );
            let client = frontend_contract::typed_client::TypedClient::new(
                TestTransport,
                frontend_contract::transport_path::TransportPath::default(),
            );
            let result = futures::executor::block_on(
                client.send_parameterized::<TestLongParameterizedRoute>(&parameter, TestRequest),
            );
            assert!(matches!(
                result,
                Err(frontend_contract::client_error::ClientError::Encode(_))
            ));
        }
    }
    #[test]
    fn test_generated_parameterized_path_keeps_valid_parameter() {
        let parameter_result = TestLongRouteParameter::try_from(constants_str::X.to_owned());
        assert!(parameter_result.is_ok());
        if let Ok(parameter) = parameter_result {
            assert_eq!(
                frontend_contract::typed_parameterized_route_path::typed_parameterized_route_path::<
                    TestLongParameterizedRoute,
                >(&parameter),
                test_long_parameterized_route(&parameter)
            );
            assert!(
                test_long_parameterized_route(&parameter)
                    .is_ok_and(|route_path| String::from(route_path).ends_with(constants_str::X))
            );
        }
    }
    #[test]
    fn test_catalog_does_not_replace_oversized_parameterized_path_with_empty_path() {
        let parameter_result = TestLongRouteParameter::try_from(constants_str::X.repeat(8192usize));
        assert!(parameter_result.is_ok());
        if let Ok(parameter) = parameter_result {
            assert!(TestParameterizedCatalog::Long(parameter)
                .catalog_path()
                .is_err_and(|error| {
                    error
                        == frontend_contract::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError::TooLong
                }));
        }
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
    fn test_route_schema_registration_initializes_shared_schemas_without_route_hooks() {
        let mut document = utoipa::openapi::OpenApi::default();
        assert!(document.components.is_none());
        frontend_contract::register_openapi_route_schemas::register_openapi_route_schemas::<
            TestDefaultHooksRoute,
        >(
            &mut frontend_contract::utoipa_open_api_ref_mut::UtoipaOpenApiRefMut::from(
                &mut document,
            ),
        );
        let expected_schemas = [
            (<frontend_contract::api_problem::ApiProblem as utoipa::ToSchema>::name(), <frontend_contract::api_problem::ApiProblem as utoipa::PartialSchema>::schema()),
            (<frontend_contract::api_problem_detail::ApiProblemDetail as utoipa::ToSchema>::name(), <frontend_contract::api_problem_detail::ApiProblemDetail as utoipa::PartialSchema>::schema()),
            (<frontend_contract::api_problem_field::ApiProblemField as utoipa::ToSchema>::name(), <frontend_contract::api_problem_field::ApiProblemField as utoipa::PartialSchema>::schema()),
            (<frontend_contract::api_problem_kind::ApiProblemKind as utoipa::ToSchema>::name(), <frontend_contract::api_problem_kind::ApiProblemKind as utoipa::PartialSchema>::schema()),
            (<frontend_contract::api_problem_request_id::ApiProblemRequestId as utoipa::ToSchema>::name(), <frontend_contract::api_problem_request_id::ApiProblemRequestId as utoipa::PartialSchema>::schema()),
            (<frontend_contract::api_problem_status::ApiProblemStatus as utoipa::ToSchema>::name(), <frontend_contract::api_problem_status::ApiProblemStatus as utoipa::PartialSchema>::schema()),
            (<frontend_contract::api_problem_violation::ApiProblemViolation as utoipa::ToSchema>::name(), <frontend_contract::api_problem_violation::ApiProblemViolation as utoipa::PartialSchema>::schema()),
            (<frontend_contract::filter_operation::FilterOperation as utoipa::ToSchema>::name(), <frontend_contract::filter_operation::FilterOperation as utoipa::PartialSchema>::schema()),
            (<frontend_contract::filter_value_shape::FilterValueShape as utoipa::ToSchema>::name(), <frontend_contract::filter_value_shape::FilterValueShape as utoipa::PartialSchema>::schema()),
        ];
        assert!(document.components.as_ref().is_some_and(|components| {
            expected_schemas
                .into_iter()
                .all(|(name, schema)| components.schemas.get(name.as_ref()) == Some(&schema))
                && !components
                    .schemas
                    .contains_key(<TestRequest as utoipa::ToSchema>::name().as_ref())
                && !components
                    .schemas
                    .contains_key(<TestResponse as utoipa::ToSchema>::name().as_ref())
        }));
    }

    #[test]
    fn test_route_schema_registration_preserves_existing_components_and_is_idempotent() {
        let mut document = utoipa::openapi::OpenApi::default();
        let mut components = utoipa::openapi::schema::Components::default();
        let unrelated_schema = <TestRequest as utoipa::PartialSchema>::schema();
        let _previous = components
            .schemas
            .insert(constants_str::ROUTE.to_owned(), unrelated_schema.clone());
        document.components = Some(components);
        frontend_contract::register_openapi_route_schemas::register_openapi_route_schemas::<
            TestRoute,
        >(
            &mut frontend_contract::utoipa_open_api_ref_mut::UtoipaOpenApiRefMut::from(
                &mut document,
            ),
        );
        assert!(
            document
                .components
                .as_ref()
                .is_some_and(|registered| registered.schemas.get(constants_str::ROUTE)
                    == Some(&unrelated_schema))
        );
        let first_registration = document.clone();
        frontend_contract::register_openapi_route_schemas::register_openapi_route_schemas::<
            TestRoute,
        >(
            &mut frontend_contract::utoipa_open_api_ref_mut::UtoipaOpenApiRefMut::from(
                &mut document,
            ),
        );
        assert!(document == first_registration);
    }

    #[test]
    fn test_schema_collections_preserve_nonempty_order_and_duplicates() {
        let values = [
            frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<
                CreatedContractTestRoute,
            >(),
            frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<
                TestRoute,
            >(),
            frontend_contract::route_schema_contract::RouteSchemaContract::from_typed_route::<
                CreatedContractTestRoute,
            >(),
        ];
        let checked = frontend_contract::route_schema_contracts::RouteSchemaContracts::try_from(
            values.to_vec(),
        );
        let statuses = |route_schema_contracts: &frontend_contract::route_schema_contracts::RouteSchemaContracts| {
            route_schema_contracts.as_ref().iter().map(|schema| schema.metadata().success_status()).collect::<Vec<_>>()
        };
        assert!(checked.is_ok_and(|route_schema_contracts| {
            let iterated =
                frontend_contract::route_schema_contracts::RouteSchemaContracts::from_max_iter(
                    values,
                );
            let expected = [
                frontend_contract::success_status::SuccessStatus::Code201,
                frontend_contract::success_status::SuccessStatus::Code200,
                frontend_contract::success_status::SuccessStatus::Code201,
            ];
            statuses(&route_schema_contracts) == expected
                && statuses(&iterated) == expected
                && iterated
                    .as_ref()
                    .iter()
                    .all(|schema| schema.response_schema().is_some())
        }));
    }

    fn assert_error_contract_replaces_stale_errors_and_retains_other_responses<Route>()
    where
        Route: frontend_contract::typed_route::TypedRoute,
    {
        let mut stale =
            utoipa::openapi::response::Response::new(constants_str::NEVER_PRINT_THIS_VALUE);
        let _previous_header = stale.headers.insert(
            constants_str::RETRY_AFTER.to_owned(),
            utoipa::openapi::header::Header::default(),
        );
        let mut operation = utoipa::openapi::path::Operation::default();
        operation.responses.responses = [200u16, 307u16, 401u16, 418u16, 429u16, 500u16, 599u16]
            .into_iter()
            .map(|status| (status.to_string(), utoipa::openapi::RefOr::T(stale.clone())))
            .collect();
        frontend_contract::apply_openapi_error_contract::apply_openapi_error_contract::<Route>(
            &mut operation,
        );
        let metadata = Route::metadata();
        assert_eq!(
            Some(operation.responses.responses.len()),
            metadata.error_statuses().len().checked_add(2usize)
        );
        [200u16, 307u16].into_iter().fold((), |(), status| {
            assert!(
                operation
                    .responses
                    .responses
                    .get(&status.to_string())
                    .is_some_and(|response_ref| {
                        serde_json::to_value(response_ref).is_ok_and(|actual| {
                            serde_json::to_value(&stale).is_ok_and(|expected| actual == expected)
                        })
                    })
            );
        });
        metadata.error_statuses().iter().copied().fold((), |(), error_status| {
            let status = error_status.transport_status().to_string();
            assert!(operation.responses.responses.get(&status).is_some_and(|response_ref| {
                match response_ref {
                    utoipa::openapi::RefOr::T(response) => {
                        let schema_matches = Route::openapi_error_response_schema(error_status).map_or_else(
                            || response.content.is_empty(),
                            |schema| response.content.len() == 1usize && response.content.get(constants_str::APPLICATION_JSON).is_some_and(|content| {
                                let expected_schema = utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from(schema);
                                content.schema.as_ref().is_some_and(|actual_schema| serde_json::to_value(actual_schema).is_ok_and(|actual| serde_json::to_value(&expected_schema).is_ok_and(|expected| actual == expected)))
                            }),
                        );
                        let rate_limited = error_status == frontend_contract::route_error_status::RouteErrorStatus::RateLimited;
                        response.description == status && schema_matches
                            && response.headers.len() == usize::from(rate_limited)
                            && response.headers.contains_key(constants_str::RETRY_AFTER) == rate_limited
                    }
                    utoipa::openapi::RefOr::Ref(_) => false,
                }
            }));
        });
    }

    #[test]
    fn test_error_contract_replaces_stale_errors_preserves_other_responses_and_respects_schema_headers()
     {
        assert_error_contract_replaces_stale_errors_and_retains_other_responses::<TestRoute>();
        assert_error_contract_replaces_stale_errors_and_retains_other_responses::<
            ErrorSchemaAbsentContractTestRoute,
        >();
        assert_error_contract_replaces_stale_errors_and_retains_other_responses::<
            TestDefaultHooksRoute,
        >();
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
    fn test_path_parameter_helper_preserves_absent_route_state_and_appends_declared_parameter() {
        let optional_parameter = <TestLongParameterizedRoute as frontend_contract::typed_route::TypedRoute>::openapi_path_parameter();
        assert!(optional_parameter.is_some_and(|parameter| {
            let expected_parameter = utoipa::openapi::path::Parameter::from(parameter);
            [None, Some(Vec::new()), Some(vec![utoipa::openapi::path::ParameterBuilder::new().name(constants_str::X).build()])].into_iter().fold((), |(), original_parameters| {
                let mut operation = utoipa::openapi::path::Operation::default();
                operation.parameters = original_parameters.clone();
                frontend_contract::apply_openapi_path_parameter_contract::apply_openapi_path_parameter_contract::<TestRoute>(&mut operation);
                assert!(serde_json::to_value(&operation.parameters).is_ok_and(|actual| serde_json::to_value(&original_parameters).is_ok_and(|expected| actual == expected)));
                let mut expected = original_parameters.unwrap_or_default();
                expected.push(expected_parameter.clone());
                frontend_contract::apply_openapi_path_parameter_contract::apply_openapi_path_parameter_contract::<TestLongParameterizedRoute>(&mut operation);
                assert!(serde_json::to_value(&operation.parameters).is_ok_and(|actual| serde_json::to_value(Some(&expected)).is_ok_and(|expected_json| actual == expected_json)));
            });
            true
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
        assert!(
            TestCatalog::Custom
                .catalog_path()
                .is_ok_and(|path| String::from(path) == constants_str::ROUTE)
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
    fn success_contract_test_metadata(
        success_status: frontend_contract::success_status::SuccessStatus,
    ) -> frontend_contract::route_metadata::RouteMetadata {
        let metadata = <TestRoute as frontend_contract::typed_route::TypedRoute>::metadata();
        frontend_contract::route_metadata::RouteMetadata::new_with_policy(
            metadata.authentication(),
            metadata.error_statuses(),
            metadata.route_method(),
            metadata.mutation(),
            metadata.openapi_operation_id(),
            metadata.path(),
            success_status,
        )
    }

    fn assert_success_contract_replaces_successes_and_retains_errors<Route>(
        success_status: frontend_contract::success_status::SuccessStatus,
        utoipa_open_api_route_schema: Option<
            frontend_contract::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema,
        >,
    ) where
        Route: frontend_contract::typed_route::TypedRoute,
    {
        let stale = utoipa::openapi::RefOr::T(utoipa::openapi::response::Response::new(
            constants_str::NEVER_PRINT_THIS_VALUE,
        ));
        let error_status = frontend_contract::transport_status::TransportStatus::from(
            frontend_contract::known_http_status::KnownHttpStatus::BadRequest,
        )
        .to_string();
        let mut operation = utoipa::openapi::path::Operation::default();
        operation.responses.responses = std::collections::BTreeMap::from([
            (
                frontend_contract::success_status::SuccessStatus::Code200
                    .transport_status()
                    .to_string(),
                stale.clone(),
            ),
            (
                frontend_contract::success_status::SuccessStatus::Code201
                    .transport_status()
                    .to_string(),
                stale.clone(),
            ),
            (
                frontend_contract::success_status::SuccessStatus::Code204
                    .transport_status()
                    .to_string(),
                stale.clone(),
            ),
            (error_status.clone(), stale),
        ]);
        frontend_contract::apply_openapi_success_contract::apply_openapi_success_contract::<Route>(
            &mut operation,
        );
        assert_eq!(operation.responses.responses.len(), 2usize);
        assert!(
            operation
                .responses
                .responses
                .get(&error_status)
                .is_some_and(|response_ref| {
                    match response_ref {
                        utoipa::openapi::RefOr::T(response) => {
                            response.description == constants_str::NEVER_PRINT_THIS_VALUE
                                && response.content.is_empty()
                        }
                        utoipa::openapi::RefOr::Ref(_) => false,
                    }
                })
        );
        let status = success_status.transport_status().to_string();
        assert!(
            operation
                .responses
                .responses
                .get(&status)
                .is_some_and(|response_ref| {
                    match response_ref {
                        utoipa::openapi::RefOr::T(response) => {
                            let schema_matches = utoipa_open_api_route_schema.map_or_else(
                                || response.content.is_empty(),
                                |schema| {
                                    response.content.len() == 1usize
                                        && response
                                            .content
                                            .get(constants_str::APPLICATION_JSON)
                                            .is_some_and(|content| {
                                                let expected_schema = utoipa::openapi::RefOr::<
                                                    utoipa::openapi::Schema,
                                                >::from(
                                                    schema
                                                );
                                                content.schema.as_ref().is_some_and(
                                                    |actual_schema| {
                                                        serde_json::to_value(actual_schema)
                                                            .is_ok_and(|actual_json| {
                                                                serde_json::to_value(
                                                                    &expected_schema,
                                                                )
                                                                .is_ok_and(|expected_json| {
                                                                    actual_json == expected_json
                                                                })
                                                            })
                                                    },
                                                )
                                            })
                                },
                            );
                            response.description == status && schema_matches
                        }
                        utoipa::openapi::RefOr::Ref(_) => false,
                    }
                })
        );
    }
    #[test]
    fn test_success_contract_replaces_stale_successes_preserves_errors_and_respects_content() {
        assert_success_contract_replaces_successes_and_retains_errors::<TestRoute>(
            frontend_contract::success_status::SuccessStatus::Code200,
            <TestRoute as frontend_contract::typed_route::TypedRoute>::openapi_response_schema(),
        );
        assert_success_contract_replaces_successes_and_retains_errors::<TestDefaultHooksRoute>(
            frontend_contract::success_status::SuccessStatus::Code200,
            None,
        );
        assert_success_contract_replaces_successes_and_retains_errors::<CreatedContractTestRoute>(
            frontend_contract::success_status::SuccessStatus::Code201,
            <TestRoute as frontend_contract::typed_route::TypedRoute>::openapi_response_schema(),
        );
        assert_success_contract_replaces_successes_and_retains_errors::<NoContentContractTestRoute>(
            frontend_contract::success_status::SuccessStatus::Code204,
            None,
        );
    }
}
