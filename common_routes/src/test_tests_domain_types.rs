#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct TestState {
    commit: &'static str,
}
impl git_info::git_commit_id_provider::GitCommitIdProvider for TestState {
    fn git_commit_id(&self) -> git_info::git_commit_id::GitCommitId {
        git_info::git_commit_id::GitCommitId::from(
            git_info::git_commit_id_ref::GitCommitIdRef::from(self.commit),
        )
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
fn test_commit_link() -> String {
    git_info::build_git_commit_link::build_git_commit_link(constants_str::TEST_VALUES_COMMIT)
        .as_ref()
        .to_owned()
}
fn test_commit_link_cow() -> git_info::git_commit_link_cow::GitCommitLinkCow {
    git_info::git_commit_link_cow::GitCommitLinkCow::try_from(std::borrow::Cow::Owned(
        test_commit_link(),
    ))
    .expect(constants_str::DIAGNOSTIC_931B775C)
}
fn b_cow(v: &'static str) -> git_info::git_commit_link_cow::GitCommitLinkCow {
    git_info::git_commit_link_cow::GitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(v))
        .expect(constants_str::DIAGNOSTIC_36301996)
}
fn uri_ref(uri: &axum::http::Uri) -> crate::axum_http_uri_ref::AxumHttpUriRef<'_> {
    crate::axum_http_uri_ref::AxumHttpUriRef::from(uri)
}
fn suffix_ref(v: &str) -> crate::uri_suffix_ref::UriSuffixRef<'_> {
    crate::uri_suffix_ref::UriSuffixRef::from(v)
}
fn assert_git_info_commit(payload: &crate::git_info::GitInfo, exp_commit: &str) {
    assert!(payload.commit_matches(exp_commit));
}
fn assert_not_found_payload_with_commit(
    payload: &crate::not_found_payload::NotFoundPayload,
    exp_commit: &str,
    exp_uri_suffix: &str,
) {
    let expected_message =
        crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
            suffix_ref(exp_uri_suffix),
        );
    assert!(payload.matches(
        exp_commit,
        &expected_message,
        constants_str::COMMON_ROUTES_SWAGGER_UI,
    ));
}
fn assert_no_route_message(actual: &to_err_string::error_text::ErrorText, uri_suffix: &str) {
    assert_eq!(
        actual.as_ref(),
        crate::make_no_route_message_for_suffix_tests::make_no_route_message_for_suffix(
            suffix_ref(uri_suffix)
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
}
#[test]
fn test_not_found_response_shape_stays_stable() {
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
        "/missing/path?limit=10"
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
    let payload = crate::make_git_info_payload_tests::make_git_info_payload(
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            state.as_ref(),
        ),
    );
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
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            state.as_ref(),
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
        "No route for "
    );
}
#[test]
fn test_make_state_payload_uses_state_trait_object() {
    let state = test_state();
    assert_eq!(
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            state.as_ref()
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
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            state.as_ref()
        )
    );
    assert_eq!(actual, format!("v={}", test_commit_link()));
}
#[test]
fn test_make_commit_json_response_combines_status_and_commit_payload() {
    let state = test_state();
    let response = crate::make_json_response::make_json_response(
        crate::make_git_info_payload_tests::make_git_info_payload(
            git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
                state.as_ref(),
            ),
        ),
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
