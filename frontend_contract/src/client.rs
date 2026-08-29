use super::create_form_value_error::create_form_value_error;
pub use super::typed_client::TypedClient;
#[cfg(test)]
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct Request {
        value: u64,
    }
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
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
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize)]
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
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct LargeRequest {
        value: String,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct Route;
    impl crate::TypedRoute for Route {
        type Request = Request;
        type Response = Response;
        type Transport = crate::PublicTransport;
        fn metadata() -> crate::RouteMetadata {
            crate::RouteMetadata::new_with_policy(
                crate::AuthenticationRequirement::Public,
                crate::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
                crate::RouteMethod::Post,
                crate::RouteMutation::Mutating,
                crate::ContractStr::from(constants_str::TEST_ALT_3),
                crate::ContractStr::from(constants_str::VALUE_AFE0CD3C),
                crate::SuccessStatus::Code200,
            )
        }
        fn request_body() -> crate::RouteRequestBody {
            crate::RouteRequestBody::Json
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct CreatedRoute;
    impl crate::TypedRoute for CreatedRoute {
        type Request = Request;
        type Response = Response;
        type Transport = crate::PublicTransport;
        fn metadata() -> crate::RouteMetadata {
            crate::RouteMetadata::new_with_policy(
                crate::AuthenticationRequirement::Public,
                crate::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
                crate::RouteMethod::Post,
                crate::RouteMutation::Mutating,
                crate::ContractStr::from(constants_str::VALUE_CC9227E7),
                crate::ContractStr::from(constants_str::VALUE_AFE0CD3C),
                crate::SuccessStatus::Code201,
            )
        }
        fn request_body() -> crate::RouteRequestBody {
            crate::RouteRequestBody::Json
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct FailingRequestRoute;
    impl crate::TypedRoute for FailingRequestRoute {
        type Request = FailingRequest;
        type Response = Response;
        type Transport = crate::PublicTransport;
        fn metadata() -> crate::RouteMetadata {
            crate::RouteMetadata::new(
                crate::RouteMethod::Post,
                crate::ContractStr::from(constants_str::VALUE_4B7BC374),
                crate::ContractStr::from(constants_str::VALUE_AFE0CD3C),
            )
        }
        fn request_body() -> crate::RouteRequestBody {
            crate::RouteRequestBody::Json
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct LargeRequestRoute;
    impl crate::TypedRoute for LargeRequestRoute {
        type Request = LargeRequest;
        type Response = Response;
        type Transport = crate::PublicTransport;
        fn metadata() -> crate::RouteMetadata {
            crate::RouteMetadata::new(
                crate::RouteMethod::Post,
                crate::ContractStr::from(constants_str::VALUE_D06CF433),
                crate::ContractStr::from(constants_str::VALUE_AFE0CD3C),
            )
        }
        fn request_body() -> crate::RouteRequestBody {
            crate::RouteRequestBody::Json
        }
    }
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct NoBody;
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct NoContentRoute;
    impl crate::TypedRoute for NoContentRoute {
        type Request = NoBody;
        type Response = NoBody;
        type Transport = crate::PublicTransport;
        fn metadata() -> crate::RouteMetadata {
            crate::RouteMetadata::new_with_policy(
                crate::AuthenticationRequirement::Public,
                crate::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
                crate::RouteMethod::Delete,
                crate::RouteMutation::Mutating,
                crate::ContractStr::from(constants_str::VALUE_E1B628F9),
                crate::ContractStr::from(constants_str::VALUE_A3F72BD5),
                crate::SuccessStatus::Code204,
            )
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
    struct EmptyOkRoute;
    impl crate::TypedRoute for EmptyOkRoute {
        type Request = NoBody;
        type Response = NoBody;
        type Transport = crate::PublicTransport;
        fn metadata() -> crate::RouteMetadata {
            crate::RouteMetadata::new(
                crate::RouteMethod::Get,
                crate::ContractStr::from(constants_str::VALUE_06DE0EB2),
                crate::ContractStr::from(constants_str::VALUE_B7407642),
            )
        }
    }
    impl crate::ParameterizedRoute for NoContentRoute {
        type Parameter = u64;
        fn path(parameter: &Self::Parameter) -> crate::ParameterizedRoutePath {
            match crate::ParameterizedRoutePath::try_from(format!("/values/{parameter}")) {
                Ok(value) => value,
                Err(error) => panic!("f7bd0a29: {error:?}"),
            }
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone)]
    struct TestTransport {
        expected: ExpectedRequest,
        response: Result<crate::TransportResponse, crate::TransportError>,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone)]
    enum ExpectedRequest {
        BodyLen(crate::TransportPath, usize),
        Empty(crate::TransportPath),
        Json(crate::TransportPath, Request),
    }
    impl crate::Transport for TestTransport {
        fn send(
            &self,
            request: crate::TransportRequest,
        ) -> impl Future<Output = Result<crate::TransportResponse, crate::TransportError>> + '_
        {
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
                        Err(error) => panic!("5f2d7a32: {error}"),
                    };
                    assert_eq!(body.value, expected_body.value);
                }
            }
            std::future::ready(self.response.clone())
        }
    }
    fn transport_path(value: &str) -> crate::TransportPath {
        match crate::TransportPath::try_from(value.to_owned()) {
            Ok(path) => path,
            Err(error) => panic!("e7222790: {error:?}"),
        }
    }
    fn response(bytes: Vec<u8>, status: crate::TransportStatus) -> crate::TransportResponse {
        let transport_body = match crate::TransportBody::try_from(bytes) {
            Ok(value) => value,
            Err(error) => panic!("05780b24: {error}"),
        };
        crate::TransportResponse::new(transport_body, status)
    }
    fn assert_static_path(prefix: &str, expected_path: &str) {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(transport_path(expected_path), Request { value: 5u64 }),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(value) => value,
                    Err(error) => panic!("f0c69ec8: {error}"),
                },
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(prefix));
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn sends_typed_route_through_transport() {
        let response_bytes = match serde_json::to_vec(&Response { value: 7u64 }) {
            Ok(value) => value,
            Err(error) => panic!("4f35f9bb: {error}"),
        };
        let response_body = match crate::TransportBody::try_from(response_bytes) {
            Ok(value) => value,
            Err(error) => panic!("d8999336: {error}"),
        };
        let expected_path =
            match crate::TransportPath::try_from(constants_str::VALUE_5B762F37.to_owned()) {
                Ok(value) => value,
                Err(error) => panic!("a805dfe8: {error:?}"),
            };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(expected_path, Request { value: 5u64 }),
            response: Ok(crate::TransportResponse::new(
                response_body,
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let prefix = match crate::TransportPath::try_from(constants_str::V1.to_owned()) {
            Ok(value) => value,
            Err(error) => panic!("b4849039: {error:?}"),
        };
        let client = super::TypedClient::new(transport, prefix);
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn sends_parameterized_route_and_decodes_empty_no_content() {
        let body = match crate::TransportBody::try_from(Vec::new()) {
            Ok(value) => value,
            Err(error) => panic!("57ef3356: {error}"),
        };
        let expected_path =
            match crate::TransportPath::try_from(constants_str::VALUE_F06110E6.to_owned()) {
                Ok(value) => value,
                Err(error) => panic!("16a72a46: {error:?}"),
            };
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(expected_path),
            response: Ok(crate::TransportResponse::new(
                body,
                crate::SuccessStatus::Code204.transport_status(),
            )),
        };
        let prefix = match crate::TransportPath::try_from(constants_str::V1_SLASH.to_owned()) {
            Ok(value) => value,
            Err(error) => panic!("e5c1d120: {error:?}"),
        };
        let client = super::TypedClient::new(transport, prefix);
        let result =
            futures::executor::block_on(client.send_parameterized::<NoContentRoute>(&9u64, NoBody));
        assert_eq!(result, Ok(NoBody));
    }
    #[test]
    fn static_absent_request_decodes_empty_ok_response() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_B7407642)),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<EmptyOkRoute>(NoBody));
        assert_eq!(result, Ok(NoBody));
    }
    #[test]
    fn path_prefix_variations_join_at_one_separator() {
        assert_static_path(
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            constants_str::VALUE_AFE0CD3C,
        );
        assert_static_path(constants_str::SLASH, constants_str::VALUE_AFE0CD3C);
        assert_static_path(constants_str::V1, constants_str::VALUE_5B762F37);
        assert_static_path(constants_str::V1_SLASH, constants_str::VALUE_5B762F37);
    }
    #[test]
    fn created_status_decodes_json_response() {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(value) => value,
                    Err(error) => panic!("03957e1b: {error}"),
                },
                crate::SuccessStatus::Code201.transport_status(),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<CreatedRoute>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn request_serialization_failure_is_an_encode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_4D1D0E01)),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<FailingRequestRoute>(FailingRequest));
        assert!(matches!(result, Err(crate::ClientError::Encode(_error))));
    }
    #[test]
    fn request_body_at_shared_limit_is_accepted() {
        let value = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - 12usize);
        let transport = TestTransport {
            expected: ExpectedRequest::BodyLen(
                transport_path(constants_str::VALUE_AFE0CD3C),
                constants_usize::VALUE_16_777_216,
            ),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(bytes) => bytes,
                    Err(error) => panic!("91bfb281: {error}"),
                },
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<LargeRequestRoute>(LargeRequest { value }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn request_body_above_shared_limit_is_an_encode_error() {
        let value = constants_str::X.repeat(constants_usize::VALUE_16_777_216 - 11usize);
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_4D1D0E01)),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result =
            futures::executor::block_on(client.send::<LargeRequestRoute>(LargeRequest { value }));
        assert!(matches!(result, Err(crate::ClientError::Encode(_error))));
    }
    #[test]
    fn joined_path_above_transport_limit_is_an_encode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path(constants_str::VALUE_4D1D0E01)),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let prefix = transport_path(format!("/{}", "x".repeat(8_191usize)).as_str());
        let client = super::TypedClient::new(transport, prefix);
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(result, Err(crate::ClientError::Encode(_error))));
    }
    #[test]
    fn malformed_success_body_is_a_decode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(
                b"{".to_vec(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(result, Err(crate::ClientError::Decode(_error))));
    }
    #[test]
    fn unexpected_status_without_problem_is_a_status_error() {
        let actual = crate::TransportStatus::from(crate::KnownHttpStatus::BadRequest);
        let expected = crate::SuccessStatus::Code200.transport_status();
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(Vec::new(), actual)),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::ClientError::Status {
                actual: actual_status,
                expected: expected_status,
            }) if actual_status == actual && expected_status == expected
        ));
    }
    #[test]
    fn api_problem_body_is_a_problem_error() {
        let problem = crate::ApiProblem::from_error(crate::ApiProblemError::Authentication);
        let problem_body = match serde_json::to_vec(&problem) {
            Ok(value) => value,
            Err(error) => panic!("0046cd3f: {error}"),
        };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Ok(response(
                problem_body,
                crate::TransportStatus::from(crate::KnownHttpStatus::Unauthorized),
            )),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::ClientError::Problem(value))
                if value.kind() == crate::ApiProblemKind::Authentication
        ));
    }
    #[test]
    fn transport_failure_is_preserved() {
        let transport_error =
            match crate::TransportError::try_from(constants_str::VALUE_8E2C7AC5.to_owned()) {
                Ok(value) => value,
                Err(error) => panic!("6d9c63f5: {error}"),
            };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(
                transport_path(constants_str::VALUE_AFE0CD3C),
                Request { value: 5u64 },
            ),
            response: Err(transport_error.clone()),
        };
        let client = super::TypedClient::new(
            transport,
            transport_path(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        );
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(result, Err(crate::ClientError::Transport(transport_error)));
    }
}
