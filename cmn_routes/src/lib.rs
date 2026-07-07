//todo gen openapi spec
const SLASH_HEALTH_CHECK: &str = "/health_check";
const SLASH_GIT_INFO: &str = "/git_info";
const SLASH_SWAGGER_UI: &str = "/swagger-ui";
const HEALTH_CHECK_SQL: &str = "SELECT 1";
const NO_ROUTE_MSG_PREFIX: &str = "No route for ";
const HEALTH_CHECK_OK_STATUS: axum::http::StatusCode = axum::http::StatusCode::OK;
const HEALTH_CHECK_ER_STATUS: axum::http::StatusCode = axum::http::StatusCode::SERVICE_UNAVAILABLE;
#[derive(Debug, serde::Serialize, optml::Optml)]
struct GitInfo {
    commit: std::borrow::Cow<'static, str>,
}
#[derive(Debug, serde::Serialize, optml::Optml)]
struct NotFoundH {
    commit: std::borrow::Cow<'static, str>,
    msg: String,
    open_api_specification: &'static str,
}
pub trait CmnRoutesPrms: git_info::GetGitCommitLink + app_state::GetPgPool + Send + Sync {}
#[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between handlers and tests
const fn mk_git_info_payload(commit: std::borrow::Cow<'static, str>) -> GitInfo {
    GitInfo { commit }
}
#[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests
fn mk_no_route_msg(uri: &axum::http::Uri) -> String {
    mk_no_route_msg_for_suffix(get_uri_suffix(uri))
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed
fn mk_no_route_msg_for_suffix(uri_suffix: &str) -> String {
    let cap = no_route_msg_capacity(uri_suffix);
    let mut msg = String::with_capacity(cap);
    msg.push_str(NO_ROUTE_MSG_PREFIX);
    msg.push_str(uri_suffix);
    msg
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and message builder
const fn no_route_msg_capacity(uri_suffix: &str) -> usize {
    NO_ROUTE_MSG_PREFIX.len().saturating_add(uri_suffix.len())
}
#[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs
fn get_uri_suffix(uri: &axum::http::Uri) -> &str {
    uri.path_and_query()
        .map_or_else(|| uri.path(), |v| v.as_str())
}
#[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place
fn mk_not_found_payload(
    uri: &axum::http::Uri,
    commit: std::borrow::Cow<'static, str>,
) -> NotFoundH {
    mk_not_found_payload_with_msg(mk_no_route_msg(uri), commit)
}
#[allow(clippy::single_call_fn)] // shared suffix-based assembly is reusable by handlers that already have a path suffix
#[cfg(test)]
fn mk_not_found_payload_for_suffix(
    uri_suffix: &str,
    commit: std::borrow::Cow<'static, str>,
) -> NotFoundH {
    mk_not_found_payload_with_msg(mk_no_route_msg_for_suffix(uri_suffix), commit)
}
#[allow(clippy::single_call_fn)] // shared payload constructor keeps not-found response shape centralized
const fn mk_not_found_payload_with_msg(
    msg: String,
    commit: std::borrow::Cow<'static, str>,
) -> NotFoundH {
    NotFoundH {
        commit,
        msg,
        open_api_specification: SLASH_SWAGGER_UI,
    }
}
#[allow(clippy::single_call_fn)] // shared helper keeps commit-based status+json responses consistent across handlers
fn mk_commit_json_res<S, T>(
    commit_src: &S,
    status: axum::http::StatusCode,
    map: impl FnOnce(std::borrow::Cow<'static, str>) -> T,
) -> (axum::http::StatusCode, axum::Json<T>)
where
    S: ?Sized + git_info::GetGitCommitLink,
{
    mk_json_res(
        status,
        map(git_info::GetGitCommitLink::get_git_commit_link_cow(
            commit_src,
        )),
    )
}
#[allow(clippy::single_call_fn)] // keeps status+json tuple construction consistent across handlers
const fn mk_json_res<T>(
    status: axum::http::StatusCode,
    payload: T,
) -> (axum::http::StatusCode, axum::Json<T>) {
    (status, axum::Json(payload))
}
#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
const fn map_health_check_status(is_ok: bool) -> axum::http::StatusCode {
    if is_ok {
        HEALTH_CHECK_OK_STATUS
    } else {
        HEALTH_CHECK_ER_STATUS
    }
}
#[allow(clippy::single_call_fn)] // named handler is clearer than inline closure for route wiring
async fn health_check(
    axum::extract::State(app_state_hc): axum::extract::State<std::sync::Arc<dyn CmnRoutesPrms>>,
) -> axum::http::StatusCode {
    map_health_check_status(
        sqlx::query(HEALTH_CHECK_SQL)
            .execute(app_state::GetPgPool::get_pg_pool(app_state_hc.as_ref()))
            .await
            .is_ok(),
    )
}
#[allow(clippy::single_call_fn)] // named handler is clearer than inline closure for route wiring
async fn git_info(
    axum::extract::State(app_state_76fb2013): axum::extract::State<
        std::sync::Arc<dyn CmnRoutesPrms>,
    >,
) -> (axum::http::StatusCode, axum::Json<GitInfo>) {
    mk_commit_json_res(
        app_state_76fb2013.as_ref(),
        axum::http::StatusCode::OK,
        mk_git_info_payload,
    )
}
#[allow(clippy::single_call_fn)] // named handler isolates fallback behavior for maintenance
async fn not_found(
    uri: axum::http::Uri,
    axum::extract::State(app_state_19103bd5): axum::extract::State<
        std::sync::Arc<dyn CmnRoutesPrms>,
    >,
) -> (axum::http::StatusCode, axum::Json<NotFoundH>) {
    mk_commit_json_res(
        app_state_19103bd5.as_ref(),
        axum::http::StatusCode::NOT_FOUND,
        |commit| mk_not_found_payload(&uri, commit),
    )
}
pub fn cmn_routes(app_state_b9fc2d94: std::sync::Arc<dyn CmnRoutesPrms>) -> axum::Router {
    axum::Router::new()
        .route(SLASH_HEALTH_CHECK, axum::routing::get(health_check))
        .route(SLASH_GIT_INFO, axum::routing::get(git_info))
        .fallback(not_found)
        .with_state(app_state_b9fc2d94)
}
#[cfg(test)]
mod tests {
    const TEST_COMMIT: &str = "abc123";
    #[derive(Debug)]
    struct TestState {
        commit: &'static str,
    }
    impl git_info::GetGitCommitId for TestState {
        fn get_git_commit_id(&self) -> String {
            self.commit.to_owned()
        }
        fn get_git_commit_id_ref(&self) -> Option<&str> {
            Some(self.commit)
        }
    }
    impl app_state::GetPgPool for TestState {
        fn get_pg_pool(&self) -> &sqlx::PgPool {
            panic!("38f80f5f")
        }
    }
    impl super::CmnRoutesPrms for TestState {}
    fn test_state() -> std::sync::Arc<dyn super::CmnRoutesPrms> {
        std::sync::Arc::new(TestState {
            commit: TEST_COMMIT,
        })
    }
    fn test_commit_link() -> String {
        git_info::git_commit_link(TEST_COMMIT)
    }
    #[allow(clippy::single_call_fn)] // shared owned->Cow conversion keeps commit-link payload setup consistent across tests
    fn test_commit_link_cow() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(test_commit_link())
    }
    const fn b_cow(v: &'static str) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(v)
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps git-info payload checks concise and consistent
    fn assert_git_info_commit(payload: &super::GitInfo, exp_commit: &str) {
        assert_eq!(payload.commit, exp_commit);
    }
    #[allow(clippy::single_call_fn)] // shared assertion centralizes not-found payload checks used across direct and state-based tests
    fn assert_not_found_payload(payload: &super::NotFoundH, exp_uri_suffix: &str) {
        assert_no_route_msg(&payload.msg, exp_uri_suffix);
        assert_eq!(payload.open_api_specification, super::SLASH_SWAGGER_UI);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps not-found commit and payload checks coupled across tests
    fn assert_not_found_payload_with_commit(
        payload: &super::NotFoundH,
        exp_commit: &str,
        exp_uri_suffix: &str,
    ) {
        assert_eq!(payload.commit, exp_commit);
        assert_not_found_payload(payload, exp_uri_suffix);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps no-route message checks consistent across uri and suffix-based tests
    fn assert_no_route_msg(actual: &str, uri_suffix: &str) {
        assert_eq!(actual, super::mk_no_route_msg_for_suffix(uri_suffix));
    }
    #[test]
    fn git_info_response_shape_stays_stable() {
        let git_info = super::mk_git_info_payload(b_cow("abc123"));
        assert_git_info_commit(&git_info, "abc123");
    }
    #[test]
    fn not_found_response_shape_stays_stable() {
        let uri = axum::http::Uri::from_static("/unknown");
        let not_found = super::mk_not_found_payload(&uri, b_cow("deadbeef"));
        assert_not_found_payload_with_commit(&not_found, "deadbeef", "/unknown");
    }
    #[test]
    fn no_route_msg_includes_uri() {
        let uri = axum::http::Uri::from_static("/missing/path");
        assert_no_route_msg(&super::mk_no_route_msg(&uri), "/missing/path");
    }
    #[test]
    fn no_route_msg_for_suffix_uses_prefix_once() {
        assert_no_route_msg(
            &super::mk_no_route_msg_for_suffix("/missing/path"),
            "/missing/path",
        );
    }
    #[test]
    fn get_uri_suffix_prefers_path_and_query_when_query_exists() {
        let uri = axum::http::Uri::from_static("/missing/path?limit=10");
        assert_eq!(super::get_uri_suffix(&uri), "/missing/path?limit=10");
    }
    #[test]
    fn no_route_msg_keeps_query_parameters() {
        let uri = axum::http::Uri::from_static("/missing/path?limit=10");
        assert_no_route_msg(&super::mk_no_route_msg(&uri), "/missing/path?limit=10");
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
        let payload = super::mk_git_info_payload(
            git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
        );
        assert_git_info_commit(&payload, test_commit_link().as_str());
    }
    #[test]
    fn not_found_response_uses_uri_and_swagger_path() {
        let uri = axum::http::Uri::from_static("/missing");
        let commit_link = test_commit_link();
        let payload = super::mk_not_found_payload(&uri, test_commit_link_cow());
        assert_not_found_payload_with_commit(&payload, &commit_link, "/missing");
    }
    #[test]
    fn not_found_payload_from_state_uses_uri_and_swagger_path() {
        let uri = axum::http::Uri::from_static("/missing");
        let state = test_state();
        let payload = super::mk_not_found_payload(
            &uri,
            git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
        );
        assert_not_found_payload_with_commit(&payload, &test_commit_link(), "/missing");
    }
    #[test]
    fn not_found_payload_for_suffix_uses_given_suffix_and_swagger_path() {
        let commit_link = test_commit_link();
        let payload = super::mk_not_found_payload_for_suffix("/missing", test_commit_link_cow());
        assert_not_found_payload_with_commit(&payload, &commit_link, "/missing");
    }
    #[test]
    fn no_route_prefix_stays_stable() {
        assert_eq!(super::NO_ROUTE_MSG_PREFIX, "No route for ");
    }
    #[test]
    fn no_route_msg_capacity_is_exact_for_uri_suffix() {
        assert_eq!(
            super::no_route_msg_capacity("/abc?x=1"),
            "No route for /abc?x=1".len()
        );
    }
    #[test]
    fn map_health_check_status_returns_ok_for_success() {
        assert_eq!(
            super::map_health_check_status(true),
            super::HEALTH_CHECK_OK_STATUS
        );
    }
    #[test]
    fn map_health_check_status_returns_unavailable_for_error() {
        assert_eq!(
            super::map_health_check_status(false),
            super::HEALTH_CHECK_ER_STATUS
        );
    }
    #[test]
    fn mk_state_payload_uses_state_trait_object() {
        let state = test_state();
        assert_eq!(
            git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
            test_commit_link()
        );
    }
    #[test]
    fn mk_json_res_wraps_payload_with_status() {
        let (status, payload) = super::mk_json_res(
            axum::http::StatusCode::CREATED,
            super::mk_git_info_payload(b_cow(TEST_COMMIT)),
        );
        assert_eq!(status, axum::http::StatusCode::CREATED);
        assert_git_info_commit(&payload.0, TEST_COMMIT);
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
        let (status, payload) = super::mk_commit_json_res(
            test_state().as_ref(),
            axum::http::StatusCode::OK,
            super::mk_git_info_payload,
        );
        assert_eq!(status, axum::http::StatusCode::OK);
        assert_git_info_commit(&payload.0, test_commit_link().as_str());
    }
}
