#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct TestState {
    commit: &'static str,
}
impl git_info::git_commit_id_provider::GitCommitIdProvider for TestState {
    fn git_commit_id(
        &self,
    ) -> Result<
        git_info::git_commit_id::GitCommitId,
        git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        git_info::git_commit_id_provider::GitCommitIdProvider::git_commit_id(self.commit)
    }
    fn git_commit_id_ref(&self) -> Option<git_info::git_commit_id_ref::GitCommitIdRef<'_>> {
        Some(git_info::git_commit_id_ref::GitCommitIdRef::from(
            self.commit,
        ))
    }
}
impl app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider for TestState {
    fn sqlx_pg_pool(&self) -> app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_> {
        std::panic::panic_any(constants_str::PANIC_38F80F5F)
    }
}
impl crate::common_routes_parameters::CommonRoutesParameters for TestState {}
fn test_state() -> std::sync::Arc<dyn crate::common_routes_parameters::CommonRoutesParameters> {
    std::sync::Arc::new(TestState {
        commit: constants_str::TEST_VALUES_COMMIT,
    })
}

#[tokio::test]
async fn test_common_state_extractor_preserves_shared_identity_and_request_parts() {
    let original = test_state();
    let state = crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(
        std::sync::Arc::clone(&original),
    );
    let clone = state.clone();
    assert!(std::ptr::eq(state.get(), original.as_ref()));
    assert!(std::ptr::eq(clone.get(), state.get()));
    let mut request = axum::http::Request::new(());
    *request.uri_mut() =
        axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10);
    *request.method_mut() = axum::http::Method::POST;
    let _previous_header = request.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static(constants_str::APPLICATION_JSON),
    );
    let (mut parts, ()) = request.into_parts();
    let result = <crate::arc_common_routes_app_state::ArcCommonRoutesAppState as axum::extract::FromRequestParts<crate::arc_common_routes_app_state::ArcCommonRoutesAppState>>::from_request_parts(&mut parts, &state).await;
    let extracted = match result {
        Ok(value) => value,
        Err(infallible) => match infallible {},
    };
    assert!(std::ptr::eq(extracted.get(), original.as_ref()));
    assert_eq!(
        parts.uri,
        axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10)
    );
    assert_eq!(parts.method, axum::http::Method::POST);
    assert_eq!(parts.headers.len(), constants_usize::ONE);
    assert_eq!(
        parts.headers.get(axum::http::header::CONTENT_TYPE),
        Some(&axum::http::HeaderValue::from_static(
            constants_str::APPLICATION_JSON
        ))
    );
}

#[test]
fn test_common_state_debug_exposes_only_wrapper_name() {
    let state = crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(test_state());
    let invalid_state = crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(
        std::sync::Arc::new(TestInvalidGitRouteState::Invalid),
    );
    [state, invalid_state].iter().fold((), |(), state_value| {
        assert_eq!(
            format!("{state_value:?}"),
            constants_str::STDARCCOMMONROUTESAPPSTATE
        );
        assert_eq!(
            format!("{state_value:#?}"),
            constants_str::STDARCCOMMONROUTESAPPSTATE
        );
    });
}

fn test_commit_link() -> String {
    test_commit_value(git_info::build_git_commit_link::build_git_commit_link(
        constants_str::TEST_VALUES_COMMIT,
    ))
    .as_ref()
    .to_owned()
}
fn test_commit_link_cow() -> git_info::git_commit_link_cow::GitCommitLinkCow {
    test_commit_value(git_info::git_commit_link_cow::GitCommitLinkCow::try_from(
        std::borrow::Cow::Owned(test_commit_link()),
    ))
}
fn b_cow(str: &'static str) -> git_info::git_commit_link_cow::GitCommitLinkCow {
    git_info::git_commit_link_cow::GitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(str))
        .expect(constants_str::DIAGNOSTIC_36301996)
}
fn uri_ref(uri: &axum::http::Uri) -> crate::axum_http_uri_ref::AxumHttpUriRef<'_> {
    crate::axum_http_uri_ref::AxumHttpUriRef::from(uri)
}
fn suffix_ref(str: &str) -> crate::uri_suffix_ref::UriSuffixRef<'_> {
    crate::uri_suffix_ref::UriSuffixRef::from(str)
}
fn assert_git_info_commit(git_info: &crate::git_info::GitInfo, str: &str) {
    assert!(git_info.commit_matches(str));
}
fn assert_not_found_payload_with_commit(
    not_found_payload: &crate::not_found_payload::NotFoundPayload,
    exp_commit: &str,
    exp_uri_suffix: &str,
) {
    let expected_message =
        crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
            suffix_ref(exp_uri_suffix),
        );
    assert!(not_found_payload.matches(
        exp_commit,
        &expected_message,
        constants_str::COMMON_ROUTES_SWAGGER_UI,
    ));
}
fn assert_no_route_message(error_text: &to_err_string::error_text::ErrorText, str: &str) {
    assert_eq!(
        error_text.as_ref(),
        crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
            suffix_ref(str)
        )
        .as_ref()
    );
}
#[test]
fn test_git_info_response_shape_stays_stable() {
    let git_info = crate::make_git_info_payload_tests::make_git_info_payload(b_cow(
        constants_str::TEST_VALUES_COMMIT,
    ));
    assert_git_info_commit(&git_info, constants_str::TEST_VALUES_COMMIT);
    let encoded = serde_json::to_value(&git_info);
    assert!(encoded.is_ok());
    let Ok(encoded_payload) = encoded else {
        return;
    };
    assert!(encoded_payload.as_object().is_some_and(|object| {
        object.len() == constants_usize::ONE
            && object.get(constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME)
                == Some(&serde_json::Value::String(
                    constants_str::TEST_VALUES_COMMIT.to_owned(),
                ))
    }));
    assert!(
        serde_json::from_value::<crate::git_info::GitInfo>(encoded_payload)
            .is_ok_and(|decoded| decoded.commit_matches(constants_str::TEST_VALUES_COMMIT))
    );
}

#[test]
fn test_git_info_payload_deserialization_preserves_commit_validation_errors() {
    let payload = |value| {
        serde_json::Value::Object(serde_json::Map::from_iter([(
            constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME.to_owned(),
            value,
        )]))
    };
    let missing = serde_json::from_value::<crate::git_info::GitInfo>(serde_json::Value::Object(
        serde_json::Map::new(),
    ));
    let expected_missing = <serde_json::Error as serde::de::Error>::missing_field(
        constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME,
    );
    assert!(missing.is_err_and(|error| error.to_string() == expected_missing.to_string()));
    [
        serde_json::Value::Null,
        serde_json::Value::Bool(false),
        serde_json::Value::Number(1u8.into()),
        serde_json::Value::Array(Vec::new()),
        serde_json::Value::Object(serde_json::Map::new()),
    ]
    .into_iter()
    .fold((), |(), value| {
        let expected = serde_json::from_value::<git_info::git_commit_link_cow::GitCommitLinkCow>(
            value.clone(),
        );
        let actual = serde_json::from_value::<crate::git_info::GitInfo>(payload(value));
        assert!(actual.is_err() && expected.is_err());
        assert_eq!(
            actual.err().map(|error| error.to_string()),
            expected.err().map(|error| error.to_string())
        );
    });
    [
        constants_usize::ZERO,
        constants_usize::VALUE_1_048_576,
        constants_usize::VALUE_1_048_576 + constants_usize::ONE,
    ]
    .into_iter()
    .fold((), |(), length| {
        let text = constants_str::X.repeat(length);
        let expected = git_info::git_commit_link_cow::GitCommitLinkCow::try_from(text.clone());
        let actual = serde_json::from_value::<crate::git_info::GitInfo>(payload(
            serde_json::Value::String(text),
        ));
        match expected {
            Ok(link) => assert!(actual.is_ok_and(|decoded| decoded.commit_matches(link.as_ref()))),
            Err(error) => {
                assert!(actual.is_err_and(|source| source.to_string() == error.to_string()));
            }
        }
    });
}
#[tokio::test]
async fn test_not_found_response_shape_stays_stable() {
    let uri = axum::http::Uri::from_static(constants_str::UNKNOWN);
    let not_found = crate::make_not_found_payload_tests::make_not_found_payload(
        uri_ref(&uri),
        b_cow(constants_str::TEST_VALUES_WRONG_COMMIT),
    );
    assert_not_found_payload_with_commit(
        &not_found,
        constants_str::TEST_VALUES_WRONG_COMMIT,
        constants_str::UNKNOWN,
    );
    let expected_message = crate::make_no_route_message_tests::make_no_route_message(uri_ref(&uri));
    let expected = serde_json::Value::Object(serde_json::Map::from_iter([
        (
            stringify!(commit).to_owned(),
            serde_json::Value::String(constants_str::TEST_VALUES_WRONG_COMMIT.to_owned()),
        ),
        (
            stringify!(message).to_owned(),
            serde_json::Value::String(expected_message.as_ref().to_owned()),
        ),
        (
            stringify!(open_api_specification).to_owned(),
            serde_json::Value::String(constants_str::COMMON_ROUTES_SWAGGER_UI.to_owned()),
        ),
    ]));
    assert!(serde_json::to_value(&not_found).is_ok_and(|encoded| encoded == expected));
    let response = axum::response::IntoResponse::into_response(
        crate::common_not_found_error::CommonNotFoundError::NotFound(not_found),
    );
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    assert_eq!(
        response.headers().get(axum::http::header::CONTENT_TYPE),
        Some(&axum::http::HeaderValue::from_static(
            constants_str::APPLICATION_JSON
        )),
    );
    let body = axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576).await;
    assert!(body.is_ok_and(|bytes| {
        serde_json::from_slice::<serde_json::Value>(&bytes).is_ok_and(|decoded| decoded == expected)
    }));
}
#[test]
fn test_no_route_message_includes_uri() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING_PATH);
    assert_no_route_message(
        &crate::make_no_route_message_tests::make_no_route_message(uri_ref(&uri)),
        constants_str::MISSING_PATH,
    );
}
#[test]
fn test_no_route_message_for_suffix_uses_prefix_once() {
    assert_no_route_message(
        &crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
            suffix_ref(constants_str::MISSING_PATH),
        ),
        constants_str::MISSING_PATH,
    );
}
#[test]
fn test_uri_suffix_prefers_path_and_query_when_query_exists() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10);
    assert_eq!(
        *crate::uri_suffix_tests::uri_suffix(uri_ref(&uri)),
        constants_str::MISSING_PATH_QUESTION_LIMIT_10
    );
}
#[test]
fn test_no_route_message_keeps_query_parameters() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10);
    assert_no_route_message(
        &crate::make_no_route_message_tests::make_no_route_message(uri_ref(&uri)),
        constants_str::MISSING_PATH_QUESTION_LIMIT_10,
    );
}
#[test]
fn test_status_code_constants_are_stable_for_common_routes() {
    assert_eq!(axum::http::StatusCode::OK.as_u16(), 200);
    assert_eq!(axum::http::StatusCode::NOT_FOUND.as_u16(), 404);
}
#[test]
fn test_git_info_response_contains_commit_link() {
    let exp_commit = test_commit_link();
    let payload = crate::make_git_info_payload_tests::make_git_info_payload(test_commit_link_cow());
    assert_git_info_commit(&payload, &exp_commit);
}
#[test]
fn test_git_info_payload_from_state_contains_commit_link() {
    let state = test_state();
    let payload = crate::make_git_info_payload_tests::make_git_info_payload(test_commit_value(
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            state.as_ref(),
        ),
    ));
    assert_git_info_commit(&payload, test_commit_link().as_str());
}
#[test]
fn test_not_found_response_uses_uri_and_swagger_path() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING);
    let commit_link = test_commit_link();
    let payload = crate::make_not_found_payload_tests::make_not_found_payload(
        uri_ref(&uri),
        test_commit_link_cow(),
    );
    assert_not_found_payload_with_commit(&payload, &commit_link, constants_str::MISSING);
}
#[test]
fn test_not_found_payload_from_state_uses_uri_and_swagger_path() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING);
    let state = test_state();
    let payload = crate::make_not_found_payload_tests::make_not_found_payload(
        uri_ref(&uri),
        test_commit_value(
            git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
                state.as_ref(),
            ),
        ),
    );
    assert_not_found_payload_with_commit(&payload, &test_commit_link(), constants_str::MISSING);
}
#[test]
fn test_not_found_payload_for_suffix_uses_given_suffix_and_swagger_path() {
    let commit_link = test_commit_link();
    let payload =
        crate::make_not_found_payload_with_message_tests::make_not_found_payload_with_message(
            crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
                suffix_ref(constants_str::MISSING),
            ),
            test_commit_link_cow(),
        );
    assert_not_found_payload_with_commit(&payload, &commit_link, constants_str::MISSING);
}
#[test]
fn test_no_route_prefix_stays_stable() {
    assert_eq!(
        constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX,
        std::str::from_utf8(b"No route for ").expect(constants_str::VALUE_2DE961C6)
    );
}
#[test]
fn test_make_state_payload_uses_state_trait_object() {
    let state = test_state();
    assert_eq!(
        test_commit_value(
            git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
                state.as_ref()
            )
        )
        .as_ref(),
        test_commit_link()
    );
}
#[test]
fn test_make_json_response_wraps_success_payload() {
    let response = crate::make_json_response::make_json_response(
        crate::make_git_info_payload_tests::make_git_info_payload(b_cow(
            constants_str::TEST_VALUES_COMMIT,
        )),
    );
    assert_git_info_commit(response.as_ref(), constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn test_make_state_payload_passes_commit_link_to_mapper() {
    let state = test_state();
    let actual = format!(
        "v={}",
        test_commit_value(
            git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
                state.as_ref()
            )
        )
    );
    assert_eq!(actual, format!("v={}", test_commit_link()));
}
#[test]
fn test_make_commit_json_response_combines_status_and_commit_payload() {
    let state = test_state();
    let response = crate::make_json_response::make_json_response(
        crate::make_git_info_payload_tests::make_git_info_payload(test_commit_value(
            git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
                state.as_ref(),
            ),
        )),
    );
    assert_git_info_commit(response.as_ref(), test_commit_link().as_str());
}
#[tokio::test]
async fn test_default_service_routes_return_success_statuses_and_match_openapi() {
    let router = axum::Router::from(crate::common_routes::common_routes(
        crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(test_state()),
    ));
    let document =
        serde_json::to_value(crate::common_routes_open_api::CommonRoutesOpenApi::open_api())
            .expect(constants_str::DIAGNOSTIC_F96BCC6E);
    let check = |path: String| {
        let cloned_router = router.clone();
        let cloned_document = document.clone();
        async move {
            let response = tower::ServiceExt::oneshot(
                cloned_router,
                axum::http::Request::builder()
                    .uri(path.as_str())
                    .body(axum::body::Body::empty())
                    .expect(constants_str::DIAGNOSTIC_6E9ABF44),
            )
            .await
            .expect(constants_str::DIAGNOSTIC_634C635B);
            assert_eq!(response.status(), axum::http::StatusCode::OK);
            assert!(
                response
                    .headers()
                    .get(axum::http::header::CONTENT_TYPE)
                    .is_some()
            );
            let escaped_path = path.replace('/', constants_str::VALUE_1_ALT_3);
            assert!(
                cloned_document
                    .pointer(format!("/paths/{escaped_path}/get/responses/200").as_str())
                    .is_some()
            );
            let body = axum::body::to_bytes(response.into_body(), 16_384usize)
                .await
                .expect(constants_str::DIAGNOSTIC_E7D5F988);
            assert!(
                serde_json::from_slice::<serde_json::Value>(&body)
                    .expect(constants_str::DIAGNOSTIC_5013A777)
                    .is_object()
            );
        }
    };
    check(
        crate::common_route::CommonRoute::HealthLive
            .path()
            .as_ref()
            .to_owned(),
    )
    .await;
    check(
        crate::common_route::CommonRoute::GitInfo
            .path()
            .as_ref()
            .to_owned(),
    )
    .await;
    crate::common_route::CommonRoute::ALL
        .into_iter()
        .for_each(|route| {
            assert!(
                route
                    .path()
                    .as_ref()
                    .ends_with(constants_str::READ_ROUTE_SUFFIX)
            );
            let escaped_path = route
                .path()
                .as_ref()
                .replace('/', constants_str::VALUE_1_ALT_3);
            assert!(
                document
                    .pointer(format!("/paths/{escaped_path}/get/responses/200").as_str())
                    .is_some()
            );
        });
    [
        crate::common_route::CommonRoute::Health,
        crate::common_route::CommonRoute::HealthCheck,
        crate::common_route::CommonRoute::HealthReady,
    ]
    .into_iter()
    .for_each(|route| {
        let escaped_path = route
            .path()
            .as_ref()
            .replace('/', constants_str::VALUE_1_ALT_3);
        assert!(
            document
                .pointer(format!("/paths/{escaped_path}/get/responses/503").as_str())
                .is_some()
        );
    });
    let not_found = tower::ServiceExt::oneshot(
        router,
        axum::http::Request::builder()
            .uri(constants_str::MISSING)
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_BB258755),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_D2B9CC45);
    assert_eq!(not_found.status(), axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_routed_fallback_preserves_query_and_exact_json_contract() {
    let router = axum::Router::from(crate::common_routes::common_routes(
        crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(test_state()),
    ));
    let mut request = axum::http::Request::new(axum::body::Body::empty());
    *request.uri_mut() =
        axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10);
    let response = match tower::ServiceExt::oneshot(router, request).await {
        Ok(response) => response,
        Err(infallible) => match infallible {},
    };
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    assert_eq!(
        response.headers().get(axum::http::header::CONTENT_TYPE),
        Some(&axum::http::HeaderValue::from_static(
            constants_str::APPLICATION_JSON
        ))
    );
    let expected = serde_json::Value::Object(serde_json::Map::from_iter([
        (
            stringify!(commit).to_owned(),
            serde_json::Value::String(test_commit_link()),
        ),
        (
            stringify!(message).to_owned(),
            serde_json::Value::String(format!(
                "{}{}",
                constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX,
                constants_str::MISSING_PATH_QUESTION_LIMIT_10
            )),
        ),
        (
            stringify!(open_api_specification).to_owned(),
            serde_json::Value::String(constants_str::COMMON_ROUTES_SWAGGER_UI.to_owned()),
        ),
    ]));
    assert!(
        axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576)
            .await
            .is_ok_and(|bytes| {
                serde_json::from_slice::<serde_json::Value>(&bytes)
                    .is_ok_and(|actual| actual == expected)
            })
    );
}

#[tokio::test]
async fn test_common_liveness_and_git_head_routes_preserve_headers_and_empty_body() {
    let router = axum::Router::from(crate::common_routes::common_routes(
        crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(test_state()),
    ));
    let check = |common_route: crate::common_route::CommonRoute| {
        let cloned_router = router.clone();
        async move {
            let request_result = axum::http::Request::builder()
                .method(axum::http::Method::HEAD)
                .uri(common_route.path().as_ref())
                .body(axum::body::Body::empty());
            assert!(request_result.is_ok());
            let Ok(request) = request_result else {
                return;
            };
            let response = match tower::ServiceExt::oneshot(cloned_router, request).await {
                Ok(response) => response,
                Err(infallible) => match infallible {},
            };
            assert_eq!(response.status(), axum::http::StatusCode::OK);
            assert_eq!(
                response.headers().get(axum::http::header::CONTENT_TYPE),
                Some(&axum::http::HeaderValue::from_static(
                    constants_str::APPLICATION_JSON
                ))
            );
            assert!(
                axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576)
                    .await
                    .is_ok_and(|bytes| bytes.is_empty())
            );
        }
    };
    check(crate::common_route::CommonRoute::HealthLive).await;
    check(crate::common_route::CommonRoute::GitInfo).await;
}

fn test_commit_value<Value>(
    result: Result<
        Value,
        git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    >,
) -> Value {
    result.expect(constants_str::DIAGNOSTIC_931B775C)
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
enum TestInvalidGitRouteState {
    Invalid,
}
impl git_info::git_commit_id_provider::GitCommitIdProvider for TestInvalidGitRouteState {
    fn git_commit_id(
        &self,
    ) -> Result<
        git_info::git_commit_id::GitCommitId,
        git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        Err(git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: constants_usize::ONE, max: constants_usize::ZERO,
        })
    }
}
impl app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider for TestInvalidGitRouteState {
    fn sqlx_pg_pool(&self) -> app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_> {
        std::panic::panic_any(constants_str::DIAGNOSTIC_8854EE29)
    }
}
impl crate::common_routes_parameters::CommonRoutesParameters for TestInvalidGitRouteState {}

#[tokio::test]
async fn test_http_adapters_propagate_commit_provider_failure() {
    let state = crate::arc_common_routes_app_state::ArcCommonRoutesAppState::from(
        std::sync::Arc::new(TestInvalidGitRouteState::Invalid),
    );
    let result = crate::git_info_response::git_info_response(state.clone()).await;
    assert!(matches!(result, Err(crate::git_info_response_error::GitInfoResponseError::CommitLink(
        git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {len: constants_usize::ONE, max: constants_usize::ZERO}
    ))));
    let router = axum::Router::from(crate::common_routes::common_routes(state));
    let mut request = axum::http::Request::new(axum::body::Body::empty());
    *request.uri_mut() = axum::http::Uri::from_static(constants_str::MISSING);
    let response = match tower::ServiceExt::oneshot(router, request).await {
        Ok(response) => response,
        Err(infallible) => match infallible {},
    };
    assert_eq!(
        response.status(),
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    );
}
