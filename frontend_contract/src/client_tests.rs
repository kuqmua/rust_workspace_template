#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct Request {
        value: u64,
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct Response {
        value: u64,
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize,
    )]
    struct FailingRequest;
    impl serde::Serialize for FailingRequest {
        fn serialize<Serializer>(
            &self,
            _serializer: Serializer,
        ) -> Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::Serializer,
        {
            Err(serde::ser::Error::custom(constants_str::VALUE_1216B447))
        }
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct LargeRequest {
        value: String,
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    enum MutatingRouteMetadataFixture {
        Ok,
        Created,
        NoContent,
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    enum BasicRouteMetadataFixture {
        FailingRequest,
        LargeRequest,
        EmptyOk,
    }
    fn basic_route_metadata(
        fixture: BasicRouteMetadataFixture,
    ) -> crate::route_metadata::RouteMetadata {
        let (method, operation, path) = match fixture {
            BasicRouteMetadataFixture::FailingRequest => (
                crate::route_method::RouteMethod::Post,
                crate::contract_str::ContractStr::from(constants_str::VALUE_4B7BC374),
                crate::contract_str::ContractStr::from(constants_str::VALUE_AFE0CD3C),
            ),
            BasicRouteMetadataFixture::LargeRequest => (
                crate::route_method::RouteMethod::Post,
                crate::contract_str::ContractStr::from(constants_str::VALUE_D06CF433),
                crate::contract_str::ContractStr::from(constants_str::VALUE_AFE0CD3C),
            ),
            BasicRouteMetadataFixture::EmptyOk => (
                crate::route_method::RouteMethod::Get,
                crate::contract_str::ContractStr::from(constants_str::VALUE_06DE0EB2),
                crate::contract_str::ContractStr::from(constants_str::VALUE_B7407642),
            ),
        };
        crate::route_metadata::RouteMetadata::new(method, operation, path)
    }
    fn mutating_route_metadata(
        fixture: MutatingRouteMetadataFixture,
    ) -> crate::route_metadata::RouteMetadata {
        let (method, operation, path, success_status) = match fixture {
            MutatingRouteMetadataFixture::Ok => (
                crate::route_method::RouteMethod::Post,
                crate::contract_str::ContractStr::from(constants_str::TEST_ALT_3),
                crate::contract_str::ContractStr::from(constants_str::VALUE_AFE0CD3C),
                crate::success_status::SuccessStatus::Code200,
            ),
            MutatingRouteMetadataFixture::Created => (
                crate::route_method::RouteMethod::Post,
                crate::contract_str::ContractStr::from(constants_str::VALUE_CC9227E7),
                crate::contract_str::ContractStr::from(constants_str::VALUE_AFE0CD3C),
                crate::success_status::SuccessStatus::Code201,
            ),
            MutatingRouteMetadataFixture::NoContent => (
                crate::route_method::RouteMethod::Delete,
                crate::contract_str::ContractStr::from(constants_str::VALUE_E1B628F9),
                crate::contract_str::ContractStr::from(constants_str::VALUE_A3F72BD5),
                crate::success_status::SuccessStatus::Code204,
            ),
        };
        crate::route_metadata::RouteMetadata::new_with_policy(
            crate::authentication_requirement::AuthenticationRequirement::Public,
            crate::route_contract::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
            method,
            crate::route_mutation::RouteMutation::Mutating,
            operation,
            path,
            success_status,
        )
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct Route;
    impl crate::typed_route::TypedRoute for Route {
        type Request = Request;
        type Response = Response;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            mutating_route_metadata(MutatingRouteMetadataFixture::Ok)
        }
        fn request_body() -> crate::route_request_body::RouteRequestBody {
            crate::route_request_body::RouteRequestBody::Json
        }
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct CreatedRoute;
    impl crate::typed_route::TypedRoute for CreatedRoute {
        type Request = Request;
        type Response = Response;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            mutating_route_metadata(MutatingRouteMetadataFixture::Created)
        }
        fn request_body() -> crate::route_request_body::RouteRequestBody {
            crate::route_request_body::RouteRequestBody::Json
        }
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct FailingRequestRoute;
    impl crate::typed_route::TypedRoute for FailingRequestRoute {
        type Request = FailingRequest;
        type Response = Response;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            basic_route_metadata(BasicRouteMetadataFixture::FailingRequest)
        }
        fn request_body() -> crate::route_request_body::RouteRequestBody {
            crate::route_request_body::RouteRequestBody::Json
        }
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct LargeRequestRoute;
    impl crate::typed_route::TypedRoute for LargeRequestRoute {
        type Request = LargeRequest;
        type Response = Response;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            basic_route_metadata(BasicRouteMetadataFixture::LargeRequest)
        }
        fn request_body() -> crate::route_request_body::RouteRequestBody {
            crate::route_request_body::RouteRequestBody::Json
        }
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct NoBody;
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct NoContentRoute;
    impl crate::typed_route::TypedRoute for NoContentRoute {
        type Request = NoBody;
        type Response = NoBody;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            mutating_route_metadata(MutatingRouteMetadataFixture::NoContent)
        }
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct EmptyOkRoute;
    impl crate::typed_route::TypedRoute for EmptyOkRoute {
        type Request = NoBody;
        type Response = NoBody;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            basic_route_metadata(BasicRouteMetadataFixture::EmptyOk)
        }
    }
    impl crate::parameterized_route::ParameterizedRoute for NoContentRoute {
        type Parameter = u64;
        fn path(
            parameter: &Self::Parameter,
        ) -> crate::parameterized_route_path::ParameterizedRoutePath {
            match crate::parameterized_route_path::ParameterizedRoutePath::try_from(format!(
                "/values/{parameter}"
            )) {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(constants_str::PANIC_F7BD0A29.replacen(
                    constants_str::PANIC_PLACEHOLDER_04CEF635,
                    format!("{error:?}").as_str(),
                    1usize,
                )),
            }
        }
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone)]
    struct TestTransport {
        expected: ExpectedRequest,
        response: Result<
            crate::transport_response::TransportResponse,
            crate::transport_error::TransportError,
        >,
    }
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone)]
    enum ExpectedRequest {
        BodyLen(crate::transport_path::TransportPath, usize),
        Empty(crate::transport_path::TransportPath),
        Json(crate::transport_path::TransportPath, Request),
    }
    impl crate::transport::Transport for TestTransport {
        fn send(
            &self,
            request: crate::transport_request::TransportRequest,
        ) -> impl Future<
            Output = Result<
                crate::transport_response::TransportResponse,
                crate::transport_error::TransportError,
            >,
        > + '_ {
            match &self.expected {
                ExpectedRequest::BodyLen(path, expected_len) => {
                    assert_eq!(request.path(), path);
                    assert_eq!(request.body().as_ref().len(), *expected_len);
                }
                ExpectedRequest::Empty(path) => {
                    assert_eq!(request.path(), path);
                    assert!(request.body().as_ref().is_empty());
                }
                ExpectedRequest::Json(path, expected_body) => {
                    assert_eq!(request.path(), path);
                    let body = match serde_json::from_slice::<Request>(request.body().as_ref()) {
                        Ok(value) => value,
                        Err(error) => {
                            std::panic::panic_any(constants_str::PANIC_5F2D7A32.replacen(
                                constants_str::PANIC_PLACEHOLDER_81240055,
                                error.to_string().as_str(),
                                1usize,
                            ))
                        }
                    };
                    assert_eq!(body.value, expected_body.value);
                }
            }
            std::future::ready(self.response.clone())
        }
    }
    fn transport_path(value: &str) -> crate::transport_path::TransportPath {
        match crate::transport_path::TransportPath::try_from(value.to_owned()) {
            Ok(path) => path,
            Err(error) => std::panic::panic_any(constants_str::PANIC_E7222790.replacen(
                constants_str::PANIC_PLACEHOLDER_04CEF635,
                format!("{error:?}").as_str(),
                1usize,
            )),
        }
    }
    fn response(
        bytes: Vec<u8>,
        status: crate::transport_status::TransportStatus,
    ) -> crate::transport_response::TransportResponse {
        let transport_body = match crate::transport_body::TransportBody::try_from(bytes) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_05780B24.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            )),
        };
        crate::transport_response::TransportResponse::new(transport_body, status)
    }
    fn assert_static_path(prefix: &str, expected_path: &str) {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(transport_path(expected_path), Request { value: 5u64 }),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(constants_str::PANIC_F0C69EC8.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    )),
                },
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(transport, transport_path(prefix));
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn test_sends_typed_route_through_transport() {
        let response_bytes = match serde_json::to_vec(&Response { value: 7u64 }) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_4F35F9BB.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            )),
        };
        let response_body = match crate::transport_body::TransportBody::try_from(response_bytes) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_D8999336.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            )),
        };
        let expected_path = match crate::transport_path::TransportPath::try_from(
            constants_str::VALUE_5B762F37.to_owned(),
        ) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_A805DFE8.replacen(
                constants_str::PANIC_PLACEHOLDER_04CEF635,
                format!("{error:?}").as_str(),
                1usize,
            )),
        };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(expected_path, Request { value: 5u64 }),
            response: Ok(crate::transport_response::TransportResponse::new(
                response_body,
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let prefix =
            match crate::transport_path::TransportPath::try_from(constants_str::V1.to_owned()) {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(constants_str::PANIC_B4849039.replacen(
                    constants_str::PANIC_PLACEHOLDER_04CEF635,
                    format!("{error:?}").as_str(),
                    1usize,
                )),
            };
        let client = crate::typed_client::TypedClient::new(transport, prefix);
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn test_sends_parameterized_route_and_decodes_empty_no_content() {
        let body = match crate::transport_body::TransportBody::try_from(Vec::new()) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_57EF3356.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            )),
        };
        let expected_path = match crate::transport_path::TransportPath::try_from(
            constants_str::VALUE_F06110E6.to_owned(),
        ) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_16A72A46.replacen(
                constants_str::PANIC_PLACEHOLDER_04CEF635,
                format!("{error:?}").as_str(),
                1usize,
            )),
        };
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(expected_path),
            response: Ok(crate::transport_response::TransportResponse::new(
                body,
                crate::success_status::SuccessStatus::Code204.transport_status(),
            )),
        };
        let prefix = match crate::transport_path::TransportPath::try_from(
            constants_str::V1_SLASH.to_owned(),
        ) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_E5C1D120.replacen(
                constants_str::PANIC_PLACEHOLDER_04CEF635,
                format!("{error:?}").as_str(),
                1usize,
            )),
        };
        let client = crate::typed_client::TypedClient::new(transport, prefix);
        let result =
            futures::executor::block_on(client.send_parameterized::<NoContentRoute>(&9u64, NoBody));
        assert_eq!(result, Ok(NoBody));
    }
    #[test]
    fn test_static_absent_request_decodes_empty_ok_response() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_B7407642)),
            response: Ok(response(
                Vec::new(),
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<EmptyOkRoute>(NoBody));
        assert_eq!(result, Ok(NoBody));
    }
    #[test]
    fn test_path_prefix_variations_join_at_one_separator() {
        assert_static_path(
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            constants_str::VALUE_AFE0CD3C,
        );
        assert_static_path(constants_str::SLASH, constants_str::VALUE_AFE0CD3C);
        assert_static_path(constants_str::V1, constants_str::VALUE_5B762F37);
        assert_static_path(constants_str::V1_SLASH, constants_str::VALUE_5B762F37);
    }
    #[test]
    fn test_created_status_decodes_json_response() {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(constants_str::PANIC_03957E1B.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    )),
                },
                crate::success_status::SuccessStatus::Code201.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<CreatedRoute>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn test_request_serialization_failure_is_an_encode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_4D1D0E01)),
            response: Ok(response(
                Vec::new(),
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<FailingRequestRoute>(FailingRequest));
        assert!(matches!(
            result,
            Err(crate::client_error::ClientError::Encode(_error))
        ));
    }
    #[test]
    fn test_request_body_at_shared_limit_is_accepted() {
        let value = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - 12usize);
        let transport = TestTransport {
            expected: ExpectedRequest::BodyLen(
                transport_path(constants_str::VALUE_AFE0CD3C),
                constants_usize::VALUE_16_777_216,
            ),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(bytes) => bytes,
                    Err(error) => std::panic::panic_any(constants_str::PANIC_91BFB281.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    )),
                },
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<LargeRequestRoute>(LargeRequest { value }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn test_request_body_above_shared_limit_is_an_encode_error() {
        let value = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - 11usize);
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_4D1D0E01)),
            response: Ok(response(
                Vec::new(),
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<LargeRequestRoute>(LargeRequest { value }));
        assert!(matches!(
            result,
            Err(crate::client_error::ClientError::Encode(_error))
        ));
    }
    #[test]
    fn test_joined_path_above_transport_limit_is_an_encode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_4D1D0E01)),
            response: Ok(response(
                Vec::new(),
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let prefix = transport_path(format!("/{}", "x".repeat(8_191usize)).as_str());
        let client = crate::typed_client::TypedClient::new(transport, prefix);
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::client_error::ClientError::Encode(_error))
        ));
    }
    #[test]
    fn test_malformed_success_body_is_a_decode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(
                b"{".to_vec(),
                crate::success_status::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::client_error::ClientError::Decode(_error))
        ));
    }
    #[test]
    fn test_unexpected_status_without_problem_is_a_status_error() {
        let actual = crate::transport_status::TransportStatus::from(
            crate::known_http_status::KnownHttpStatus::BadRequest,
        );
        let expected = crate::success_status::SuccessStatus::Code200.transport_status();
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(Vec::new(), actual)),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::client_error::ClientError::Status {
                actual: actual_status,
                expected: expected_status,
            }) if actual_status == actual && expected_status == expected
        ));
    }
    #[test]
    fn test_api_problem_body_is_a_problem_error() {
        let problem = crate::api_problem::ApiProblem::from_error(
            crate::api_problem_error::ApiProblemError::Authentication,
        );
        let problem_body = match serde_json::to_vec(&problem) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_0046CD3F.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            )),
        };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(
                problem_body,
                crate::transport_status::TransportStatus::from(
                    crate::known_http_status::KnownHttpStatus::Unauthorized,
                ),
            )),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::client_error::ClientError::Problem(value))
                if value.kind() == crate::api_problem_kind::ApiProblemKind::Authentication
        ));
    }
    #[test]
    fn test_transport_failure_is_preserved() {
        let transport_error = match crate::transport_error::TransportError::try_from(
            constants_str::VALUE_8E2C7AC5.to_owned(),
        ) {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(constants_str::PANIC_6D9C63F5.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            )),
        };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Err(transport_error.clone()),
        };
        let client = crate::typed_client::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(
            result,
            Err(crate::client_error::ClientError::Transport(transport_error))
        );
    }
}
