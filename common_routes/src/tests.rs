#[derive(optml::Optml, Debug)]
struct TestState {
    commit: &'static str,
}
impl git_info::GetGitCommitId for TestState {
    fn get_git_commit_id(&self) -> git_info::GitCommitId {
        git_info::GitCommitId::from(git_info::GitCommitIdRef::from(self.commit))
    }
    fn get_git_commit_id_ref(&self) -> Option<git_info::GitCommitIdRef<'_>> {
        Some(git_info::GitCommitIdRef::from(self.commit))
    }
}
impl app_state::GetSqlxPgPool for TestState {
    fn get_sqlx_pg_pool(&self) -> app_state::SqlxPgPoolRef<'_> {
        panic!("38f80f5f")
    }
}
impl super::CommonRoutesParameters for TestState {}
fn test_state() -> std::sync::Arc<dyn super::CommonRoutesParameters> {
    std::sync::Arc::new(TestState {
        commit: str_constants::TEST_VALUES_COMMIT,
    })
}
fn test_commit_link() -> String {
    git_info::git_commit_link(str_constants::TEST_VALUES_COMMIT)
        .as_ref()
        .to_owned()
}
fn test_commit_link_cow() -> git_info::StdGitCommitLinkCow {
    git_info::StdGitCommitLinkCow::try_from(std::borrow::Cow::Owned(test_commit_link()))
        .expect("931b775c")
}
fn b_cow(v: &'static str) -> git_info::StdGitCommitLinkCow {
    git_info::StdGitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(v)).expect("36301996")
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
    payload: &super::NotFoundHandle,
    exp_commit: &str,
    exp_uri_suffix: &str,
) {
    assert_eq!(payload.commit.as_ref(), exp_commit);
    assert_no_route_message(&payload.message, exp_uri_suffix);
    assert_eq!(
        payload.open_api_specification.0,
        str_constants::COMMON_ROUTES_SWAGGER_UI
    );
}
fn assert_no_route_message(actual: &to_err_string::ErrorText, uri_suffix: &str) {
    assert_eq!(
        actual.as_ref(),
        super::mk_no_route_message_for_suffix(suffix_ref(uri_suffix)).as_ref()
    );
}
#[test]
fn git_info_response_shape_stays_stable() {
    let git_info = super::mk_git_info_payload(b_cow(str_constants::TEST_VALUES_COMMIT));
    assert_git_info_commit(&git_info, str_constants::TEST_VALUES_COMMIT);
}
#[test]
fn not_found_response_shape_stays_stable() {
    let uri = axum::http::Uri::from_static(str_constants::UNKNOWN);
    let not_found = super::mk_not_found_payload(
        uri_ref(&uri),
        b_cow(str_constants::TEST_VALUES_WRONG_COMMIT),
    );
    assert_not_found_payload_with_commit(
        &not_found,
        str_constants::TEST_VALUES_WRONG_COMMIT,
        str_constants::UNKNOWN,
    );
}
#[test]
fn no_route_message_includes_uri() {
    let uri = axum::http::Uri::from_static(str_constants::MISSING_PATH);
    assert_no_route_message(
        &super::mk_no_route_message(uri_ref(&uri)),
        str_constants::MISSING_PATH,
    );
}
#[test]
fn no_route_message_for_suffix_uses_prefix_once() {
    assert_no_route_message(
        &super::mk_no_route_message_for_suffix(suffix_ref(str_constants::MISSING_PATH)),
        str_constants::MISSING_PATH,
    );
}
#[test]
fn get_uri_suffix_prefers_path_and_query_when_query_exists() {
    let uri = axum::http::Uri::from_static(str_constants::MISSING_PATH_QUESTION_LIMIT_10);
    assert_eq!(
        super::get_uri_suffix(uri_ref(&uri)).0,
        "/missing/path?limit=10"
    );
}
#[test]
fn no_route_message_keeps_query_parameters() {
    let uri = axum::http::Uri::from_static(str_constants::MISSING_PATH_QUESTION_LIMIT_10);
    assert_no_route_message(
        &super::mk_no_route_message(uri_ref(&uri)),
        str_constants::MISSING_PATH_QUESTION_LIMIT_10,
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
    let payload = super::mk_git_info_payload(test_commit_link_cow());
    assert_git_info_commit(&payload, &exp_commit);
}
#[test]
fn git_info_payload_from_state_contains_commit_link() {
    let state = test_state();
    let payload = super::mk_git_info_payload(git_info::GetGitCommitLink::get_git_commit_link_cow(
        state.as_ref(),
    ));
    assert_git_info_commit(&payload, test_commit_link().as_str());
}
#[test]
fn not_found_response_uses_uri_and_swagger_path() {
    let uri = axum::http::Uri::from_static(str_constants::MISSING);
    let commit_link = test_commit_link();
    let payload = super::mk_not_found_payload(uri_ref(&uri), test_commit_link_cow());
    assert_not_found_payload_with_commit(&payload, &commit_link, str_constants::MISSING);
}
#[test]
fn not_found_payload_from_state_uses_uri_and_swagger_path() {
    let uri = axum::http::Uri::from_static(str_constants::MISSING);
    let state = test_state();
    let payload = super::mk_not_found_payload(
        uri_ref(&uri),
        git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
    );
    assert_not_found_payload_with_commit(&payload, &test_commit_link(), str_constants::MISSING);
}
#[test]
fn not_found_payload_for_suffix_uses_given_suffix_and_swagger_path() {
    let commit_link = test_commit_link();
    let payload = super::mk_not_found_payload_with_message(
        super::mk_no_route_message_for_suffix(suffix_ref(str_constants::MISSING)),
        test_commit_link_cow(),
    );
    assert_not_found_payload_with_commit(&payload, &commit_link, str_constants::MISSING);
}
#[test]
fn no_route_prefix_stays_stable() {
    assert_eq!(
        str_constants::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX,
        "No route for "
    );
}
#[test]
fn no_route_message_capacity_is_exact_for_uri_suffix() {
    assert_eq!(
        super::no_route_message_capacity(suffix_ref("/abc?x=1")).0,
        "No route for /abc?x=1".len()
    );
}
#[test]
fn mk_state_payload_uses_state_trait_object() {
    let state = test_state();
    assert_eq!(
        git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()).as_ref(),
        test_commit_link()
    );
}
#[test]
fn mk_json_res_wraps_success_payload() {
    let response = super::mk_json_res(super::mk_git_info_payload(b_cow(
        str_constants::TEST_VALUES_COMMIT,
    )));
    assert_git_info_commit(&response.payload.0, str_constants::TEST_VALUES_COMMIT);
}
#[test]
fn mk_state_payload_passes_commit_link_to_mapper() {
    let state = test_state();
    let actual = format!(
        "v={}",
        git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref())
    );
    assert_eq!(actual, format!("v={}", test_commit_link()));
}
#[test]
fn mk_commit_json_res_combines_status_and_commit_payload() {
    let response = super::mk_commit_json_res(test_state().as_ref(), super::mk_git_info_payload);
    assert_git_info_commit(&response.payload.0, test_commit_link().as_str());
}
#[tokio::test]
async fn default_service_routes_return_success_statuses_and_match_openapi() {
    let router = axum::Router::from(super::common_routes(
        super::StdArcCommonRoutesAppState::from(test_state()),
    ));
    let document = serde_json::to_value(super::CommonRoutesOpenApi::open_api()).expect("f96bcc6e");
    let check = |path: String| {
        let cloned_router = router.clone();
        let cloned_document = document.clone();
        async move {
            let response = tower::ServiceExt::oneshot(
                cloned_router,
                axum::http::Request::builder()
                    .uri(path.as_str())
                    .body(axum::body::Body::empty())
                    .expect("6e9abf44"),
            )
            .await
            .expect("634c635b");
            assert_eq!(response.status(), axum::http::StatusCode::OK);
            assert!(
                response
                    .headers()
                    .get(axum::http::header::CONTENT_TYPE)
                    .is_some()
            );
            let escaped_path = path.replace('/', str_constants::VALUE_1_ALT_3);
            assert!(
                cloned_document
                    .pointer(format!("/paths/{escaped_path}/get/responses/200").as_str())
                    .is_some()
            );
            let body = axum::body::to_bytes(response.into_body(), 16_384usize)
                .await
                .expect("e7d5f988");
            assert!(
                serde_json::from_slice::<serde_json::Value>(&body)
                    .expect("5013a777")
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
            .replace('/', str_constants::VALUE_1_ALT_3);
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
            .replace('/', str_constants::VALUE_1_ALT_3);
        assert!(
            document
                .pointer(format!("/paths/{escaped_path}/get/responses/503").as_str())
                .is_some()
        );
    });
    let not_found = tower::ServiceExt::oneshot(
        router,
        axum::http::Request::builder()
            .uri(str_constants::MISSING)
            .body(axum::body::Body::empty())
            .expect("bb258755"),
    )
    .await
    .expect("d2b9cc45");
    assert_eq!(not_found.status(), axum::http::StatusCode::NOT_FOUND);
}
mod health;
mod route_contract;
