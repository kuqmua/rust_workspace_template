#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct TestState {
    commit: &'static str,
}
impl git_info::domain_types::GitCommitIdProvider for TestState {
    fn git_commit_id(&self) -> git_info::domain_types::GitCommitId {
        git_info::domain_types::GitCommitId::from(git_info::domain_types::GitCommitIdRef::from(
            self.commit,
        ))
    }
    fn git_commit_id_ref(&self) -> Option<git_info::domain_types::GitCommitIdRef<'_>> {
        Some(git_info::domain_types::GitCommitIdRef::from(self.commit))
    }
}
impl app_state::domain_types::SqlxPgPoolProvider for TestState {
    fn sqlx_pg_pool(&self) -> app_state::domain_types::SqlxPgPoolRef<'_> {
        panic!("38f80f5f")
    }
}
impl super::CommonRoutesParameters for TestState {}
fn test_state() -> std::sync::Arc<dyn super::CommonRoutesParameters> {
    std::sync::Arc::new(TestState {
        commit: constants_str::TEST_VALUES_COMMIT,
    })
}
fn test_commit_link() -> String {
    git_info::domain_types::build_git_commit_link(constants_str::TEST_VALUES_COMMIT)
        .as_ref()
        .to_owned()
}
fn test_commit_link_cow() -> git_info::domain_types::GitCommitLinkCow {
    git_info::domain_types::GitCommitLinkCow::try_from(std::borrow::Cow::Owned(test_commit_link()))
        .expect("931b775c test_commit_link_cow invariant must hold")
}
fn b_cow(v: &'static str) -> git_info::domain_types::GitCommitLinkCow {
    git_info::domain_types::GitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(v))
        .expect("36301996 b_cow invariant must hold")
}
fn uri_ref(uri: &axum::http::Uri) -> super::AxumHttpUriRef<'_> {
    super::AxumHttpUriRef::from(uri)
}
fn suffix_ref(v: &str) -> super::UriSuffixRef<'_> {
    super::UriSuffixRef::from(v)
}
fn assert_git_info_commit(payload: &super::GitInfo, exp_commit: &str) {
    assert_eq!(payload.commit.as_ref(), exp_commit);
}
fn assert_not_found_payload_with_commit(
    payload: &super::NotFoundPayload,
    exp_commit: &str,
    exp_uri_suffix: &str,
) {
    assert_eq!(payload.commit.as_ref(), exp_commit);
    assert_no_route_message(&payload.message, exp_uri_suffix);
    assert_eq!(
        payload.open_api_specification.0,
        constants_str::COMMON_ROUTES_SWAGGER_UI
    );
}
fn assert_no_route_message(actual: &to_err_string::domain_types::ErrorText, uri_suffix: &str) {
    assert_eq!(
        actual.as_ref(),
        super::make_no_route_message_for_suffix(suffix_ref(uri_suffix)).as_ref()
    );
}
#[test]
fn git_info_response_shape_stays_stable() {
    let git_info = super::make_git_info_payload(b_cow(constants_str::TEST_VALUES_COMMIT));
    assert_git_info_commit(&git_info, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn not_found_response_shape_stays_stable() {
    let uri = axum::http::Uri::from_static(constants_str::UNKNOWN);
    let not_found = super::make_not_found_payload(
        uri_ref(&uri),
        b_cow(constants_str::TEST_VALUES_WRONG_COMMIT),
    );
    assert_not_found_payload_with_commit(
        &not_found,
        constants_str::TEST_VALUES_WRONG_COMMIT,
        constants_str::UNKNOWN,
    );
}
#[test]
fn no_route_message_includes_uri() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING_PATH);
    assert_no_route_message(
        &super::make_no_route_message(uri_ref(&uri)),
        constants_str::MISSING_PATH,
    );
}
#[test]
fn no_route_message_for_suffix_uses_prefix_once() {
    assert_no_route_message(
        &super::make_no_route_message_for_suffix(suffix_ref(constants_str::MISSING_PATH)),
        constants_str::MISSING_PATH,
    );
}
#[test]
fn uri_suffix_prefers_path_and_query_when_query_exists() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10);
    assert_eq!(super::uri_suffix(uri_ref(&uri)).0, "/missing/path?limit=10");
}
#[test]
fn no_route_message_keeps_query_parameters() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING_PATH_QUESTION_LIMIT_10);
    assert_no_route_message(
        &super::make_no_route_message(uri_ref(&uri)),
        constants_str::MISSING_PATH_QUESTION_LIMIT_10,
    );
}
#[test]
fn status_code_constants_are_stable_for_common_routes() {
    assert_eq!(axum::http::StatusCode::OK.as_u16(), 200);
    assert_eq!(axum::http::StatusCode::NOT_FOUND.as_u16(), 404);
}
#[test]
fn git_info_response_contains_commit_link() {
    let exp_commit = test_commit_link();
    let payload = super::make_git_info_payload(test_commit_link_cow());
    assert_git_info_commit(&payload, &exp_commit);
}
#[test]
fn git_info_payload_from_state_contains_commit_link() {
    let state = test_state();
    let payload = super::make_git_info_payload(
        git_info::domain_types::GitCommitLinkProvider::build_git_commit_link_cow(state.as_ref()),
    );
    assert_git_info_commit(&payload, test_commit_link().as_str());
}
#[test]
fn not_found_response_uses_uri_and_swagger_path() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING);
    let commit_link = test_commit_link();
    let payload = super::make_not_found_payload(uri_ref(&uri), test_commit_link_cow());
    assert_not_found_payload_with_commit(&payload, &commit_link, constants_str::MISSING);
}
#[test]
fn not_found_payload_from_state_uses_uri_and_swagger_path() {
    let uri = axum::http::Uri::from_static(constants_str::MISSING);
    let state = test_state();
    let payload = super::make_not_found_payload(
        uri_ref(&uri),
        git_info::domain_types::GitCommitLinkProvider::build_git_commit_link_cow(state.as_ref()),
    );
    assert_not_found_payload_with_commit(&payload, &test_commit_link(), constants_str::MISSING);
}
#[test]
fn not_found_payload_for_suffix_uses_given_suffix_and_swagger_path() {
    let commit_link = test_commit_link();
    let payload = super::make_not_found_payload_with_message(
        super::make_no_route_message_for_suffix(suffix_ref(constants_str::MISSING)),
        test_commit_link_cow(),
    );
    assert_not_found_payload_with_commit(&payload, &commit_link, constants_str::MISSING);
}
#[test]
fn no_route_prefix_stays_stable() {
    assert_eq!(
        constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX,
        "No route for "
    );
}
#[test]
fn make_state_payload_uses_state_trait_object() {
    let state = test_state();
    assert_eq!(
        git_info::domain_types::GitCommitLinkProvider::build_git_commit_link_cow(state.as_ref())
            .as_ref(),
        test_commit_link()
    );
}
#[test]
fn make_json_response_wraps_success_payload() {
    let response = super::make_json_response(super::make_git_info_payload(b_cow(
        constants_str::TEST_VALUES_COMMIT,
    )));
    assert_git_info_commit(&response.payload.0, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn make_state_payload_passes_commit_link_to_mapper() {
    let state = test_state();
    let actual = format!(
        "v={}",
        git_info::domain_types::GitCommitLinkProvider::build_git_commit_link_cow(state.as_ref())
    );
    assert_eq!(actual, format!("v={}", test_commit_link()));
}
#[test]
fn make_commit_json_response_combines_status_and_commit_payload() {
    let response =
        super::make_commit_json_response(test_state().as_ref(), super::make_git_info_payload);
    assert_git_info_commit(&response.payload.0, test_commit_link().as_str());
}
#[tokio::test]
async fn default_service_routes_return_success_statuses_and_match_openapi() {
    let router = axum::Router::from(crate::adapters::common_routes(
        super::ArcCommonRoutesAppState::from(test_state()),
    ));
    let document = serde_json::to_value(super::CommonRoutesOpenApi::open_api()).expect("f96bcc6e default_service_routes_return_success_statuses_and_match_openapi invariant must hold");
    let check = |path: String| {
        let cloned_router = router.clone();
        let cloned_document = document.clone();
        async move {
            let response = tower::ServiceExt::oneshot(
                cloned_router,
                axum::http::Request::builder()
                    .uri(path.as_str())
                    .body(axum::body::Body::empty())
                    .expect("6e9abf44 default_service_routes_return_success_statuses_and_match_openapi invariant must hold"),
            )
            .await
            .expect("634c635b default_service_routes_return_success_statuses_and_match_openapi invariant must hold");
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
                .expect("e7d5f988 default_service_routes_return_success_statuses_and_match_openapi invariant must hold");
            assert!(
                serde_json::from_slice::<serde_json::Value>(&body)
                    .expect("5013a777 default_service_routes_return_success_statuses_and_match_openapi invariant must hold")
                    .is_object()
            );
        }
    };
    check(super::CommonRoute::HealthLive.path().as_ref().to_owned()).await;
    check(super::CommonRoute::GitInfo.path().as_ref().to_owned()).await;
    super::CommonRoute::ALL.into_iter().for_each(|route| {
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
        super::CommonRoute::Health,
        super::CommonRoute::HealthCheck,
        super::CommonRoute::HealthReady,
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
            .expect("bb258755 default_service_routes_return_success_statuses_and_match_openapi invariant must hold"),
    )
    .await
    .expect("d2b9cc45 default_service_routes_return_success_statuses_and_match_openapi invariant must hold");
    assert_eq!(not_found.status(), axum::http::StatusCode::NOT_FOUND);
}
