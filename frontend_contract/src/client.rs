#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct TypedClient<Transport> {
    path_prefix: crate::TransportPath,
    transport: Transport,
}
impl<Transport> TypedClient<Transport>
where
    Transport: crate::Transport,
{
    #[must_use]
    pub const fn new(transport: Transport, path_prefix: crate::TransportPath) -> Self {
        Self {
            path_prefix,
            transport,
        }
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    pub async fn send<Route>(
        &self,
        body: Route::Request,
    ) -> Result<Route::Response, crate::ClientError>
    where
        Route: crate::TypedRoute,
    {
        let route_path =
            crate::TransportPath::try_from(Route::metadata().path().as_ref().to_owned())
                .map_err(|error| crate::ClientError::Encode(form_value_error(error)))?;
        self.send_to::<Route>(&route_path, body).await
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    pub async fn send_parameterized<Route>(
        &self,
        parameter: &Route::Parameter,
        body: Route::Request,
    ) -> Result<Route::Response, crate::ClientError>
    where
        Route: crate::ParameterizedRoute,
    {
        let route_path = crate::TransportPath::try_from(String::from(Route::path(parameter)))
            .map_err(|error| crate::ClientError::Encode(form_value_error(error)))?;
        self.send_to::<Route>(&route_path, body).await
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    pub async fn send_contract(
        &self,
        contract: crate::RouteContract,
        route_path: crate::ContractStr,
    ) -> Result<crate::TransportBody, crate::ClientError> {
        let transport_path = crate::TransportPath::try_from(route_path.as_ref().to_owned())
            .map_err(|error| crate::ClientError::Encode(form_value_error(error)))?;
        let transport_body = crate::TransportBody::try_from(Vec::new())
            .map_err(|error| crate::ClientError::Encode(form_value_error(error)))?;
        let response = self
            .send_request(transport_body, &transport_path, contract)
            .await?;
        response
            .success_body(contract.success_status().transport_status())
            .cloned()
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    async fn send_to<Route>(
        &self,
        route_path: &crate::TransportPath,
        body: Route::Request,
    ) -> Result<Route::Response, crate::ClientError>
    where
        Route: crate::TypedRoute,
    {
        let metadata = Route::metadata();
        let transport_body = match Route::request_body() {
            crate::RouteRequestBody::Absent => crate::TransportBody::try_from(Vec::new()),
            crate::RouteRequestBody::Json => serde_json::to_vec(&body)
                .map_err(|error| crate::ClientError::Encode(form_value_error(error)))?
                .try_into(),
        }
        .map_err(|error: crate::FrontendContractBodyError| {
            crate::ClientError::Encode(form_value_error(error))
        })?;
        let response = self
            .send_request(transport_body, route_path, metadata.contract())
            .await?;
        let response_body = response.success_body(metadata.success_status().transport_status())?;
        let bytes = if response_body.as_ref().is_empty() {
            b"null".as_slice()
        } else {
            response_body.as_ref()
        };
        serde_json::from_slice(bytes)
            .map_err(|error| crate::ClientError::Decode(form_value_error(error)))
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    async fn send_request(
        &self,
        body: crate::TransportBody,
        route_path: &crate::TransportPath,
        contract: crate::RouteContract,
    ) -> Result<crate::TransportResponse, crate::ClientError> {
        let prefix_ref = self.path_prefix.as_ref().trim_end_matches('/');
        let route_path_ref = route_path.as_ref().trim_start_matches('/');
        let path_string = if prefix_ref.is_empty() {
            format!("/{route_path_ref}")
        } else if route_path_ref.is_empty() {
            prefix_ref.to_owned()
        } else {
            format!("{prefix_ref}/{route_path_ref}")
        };
        let path = crate::TransportPath::try_from(path_string)
            .map_err(|error| crate::ClientError::Encode(form_value_error(error)))?;
        self.transport
            .send(crate::TransportRequest::new(body, path, contract))
            .await
            .map_err(crate::ClientError::Transport)
    }
}
fn form_value_error(error: impl std::fmt::Display) -> crate::FormValueError {
    crate::FormValueError::try_from(error.to_string()).unwrap_or_default()
}

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
            Err(serde::ser::Error::custom("request serialization failed"))
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
                crate::ContractStr::from("test"),
                crate::ContractStr::from("/values"),
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
                crate::ContractStr::from("create_test"),
                crate::ContractStr::from("/values"),
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
                crate::ContractStr::from("failing_test"),
                crate::ContractStr::from("/values"),
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
                crate::ContractStr::from("large_test"),
                crate::ContractStr::from("/values"),
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
                crate::ContractStr::from("delete_test"),
                crate::ContractStr::from("/values/{value}"),
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
                crate::ContractStr::from("empty_ok_test"),
                crate::ContractStr::from("/health_check"),
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
        let expected_path = match crate::TransportPath::try_from("/v1/values".to_owned()) {
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
        let prefix = match crate::TransportPath::try_from("/v1".to_owned()) {
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
        let expected_path = match crate::TransportPath::try_from("/v1/values/9".to_owned()) {
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
        let prefix = match crate::TransportPath::try_from("/v1/".to_owned()) {
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
            expected: ExpectedRequest::Empty(transport_path("/health_check")),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result = futures::executor::block_on(client.send::<EmptyOkRoute>(NoBody));
        assert_eq!(result, Ok(NoBody));
    }
    #[test]
    fn path_prefix_variations_join_at_one_separator() {
        assert_static_path("", "/values");
        assert_static_path("/", "/values");
        assert_static_path("/v1", "/v1/values");
        assert_static_path("/v1/", "/v1/values");
    }
    #[test]
    fn created_status_decodes_json_response() {
        let transport = TestTransport {
            expected: ExpectedRequest::Json(transport_path("/values"), Request { value: 5u64 }),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(value) => value,
                    Err(error) => panic!("03957e1b: {error}"),
                },
                crate::SuccessStatus::Code201.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result =
            futures::executor::block_on(client.send::<CreatedRoute>(Request { value: 5u64 }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn request_serialization_failure_is_an_encode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path("/unused")),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result =
            futures::executor::block_on(client.send::<FailingRequestRoute>(FailingRequest));
        assert!(matches!(result, Err(crate::ClientError::Encode(_error))));
    }
    #[test]
    fn request_body_at_shared_limit_is_accepted() {
        let value = "x".repeat(crate::FRONTEND_CONTRACT_BODY_MAX_BYTES - 12usize);
        let transport = TestTransport {
            expected: ExpectedRequest::BodyLen(
                transport_path("/values"),
                crate::FRONTEND_CONTRACT_BODY_MAX_BYTES,
            ),
            response: Ok(response(
                match serde_json::to_vec(&Response { value: 7u64 }) {
                    Ok(bytes) => bytes,
                    Err(error) => panic!("91bfb281: {error}"),
                },
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result =
            futures::executor::block_on(client.send::<LargeRequestRoute>(LargeRequest { value }));
        assert_eq!(result, Ok(Response { value: 7u64 }));
    }
    #[test]
    fn request_body_above_shared_limit_is_an_encode_error() {
        let value = "x".repeat(crate::FRONTEND_CONTRACT_BODY_MAX_BYTES - 11usize);
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path("/unused")),
            response: Ok(response(
                Vec::new(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result =
            futures::executor::block_on(client.send::<LargeRequestRoute>(LargeRequest { value }));
        assert!(matches!(result, Err(crate::ClientError::Encode(_error))));
    }
    #[test]
    fn joined_path_above_transport_limit_is_an_encode_error() {
        let transport = TestTransport {
            expected: ExpectedRequest::Empty(transport_path("/unused")),
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
            expected: ExpectedRequest::Json(transport_path("/values"), Request { value: 5u64 }),
            response: Ok(response(
                b"{".to_vec(),
                crate::SuccessStatus::Code200.transport_status(),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(result, Err(crate::ClientError::Decode(_error))));
    }
    #[test]
    fn unexpected_status_without_problem_is_a_status_error() {
        let actual = crate::TransportStatus::from(crate::KnownHttpStatus::BadRequest);
        let expected = crate::SuccessStatus::Code200.transport_status();
        let transport = TestTransport {
            expected: ExpectedRequest::Json(transport_path("/values"), Request { value: 5u64 }),
            response: Ok(response(Vec::new(), actual)),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
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
            expected: ExpectedRequest::Json(transport_path("/values"), Request { value: 5u64 }),
            response: Ok(response(
                problem_body,
                crate::TransportStatus::from(crate::KnownHttpStatus::Unauthorized),
            )),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert!(matches!(
            result,
            Err(crate::ClientError::Problem(value))
                if value.kind() == crate::ApiProblemKind::Authentication
        ));
    }
    #[test]
    fn transport_failure_is_preserved() {
        let transport_error = match crate::TransportError::try_from("offline".to_owned()) {
            Ok(value) => value,
            Err(error) => panic!("6d9c63f5: {error}"),
        };
        let transport = TestTransport {
            expected: ExpectedRequest::Json(transport_path("/values"), Request { value: 5u64 }),
            response: Err(transport_error.clone()),
        };
        let client = super::TypedClient::new(transport, transport_path(""));
        let result = futures::executor::block_on(client.send::<Route>(Request { value: 5u64 }));
        assert_eq!(result, Err(crate::ClientError::Transport(transport_error)));
    }
}
