#![allow(
    clippy::needless_for_each,
    reason = "utoipa OpenApi derive expands to an internal for_each"
)]
//todo gen openapi spec
const SLASH_HEALTH_CHECK: &str = "/health_check";
const SLASH_HEALTH: &str = "/health";
const SLASH_HEALTH_LIVE: &str = "/health/live";
const SLASH_HEALTH_READY: &str = "/health/ready";
const SLASH_GIT_INFO: &str = "/git_info";
const SLASH_SWAGGER_UI: &str = "/swagger-ui";
const HEALTH_CHECK_SQL: &str = "SELECT 1";
const HEALTH_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2u64);
const NO_ROUTE_MSG_PREFIX: &str = "No route for ";
const NOT_FOUND_MSG_MAX_LEN: usize = 1_048_576;
const HEALTH_CHECK_OK_STATUS: AxumHealthCheckStatus =
    AxumHealthCheckStatus(axum::http::StatusCode::OK);
const HEALTH_CHECK_ER_STATUS: AxumHealthCheckStatus =
    AxumHealthCheckStatus(axum::http::StatusCode::SERVICE_UNAVAILABLE);
#[derive(Debug, serde::Serialize, utoipa::ToSchema, optml::Optml)]
pub struct GitInfo {
    commit: git_info::StdGitCommitLinkCow,
}
#[derive(Debug, serde::Serialize, optml::Optml)]
struct NotFoundH {
    commit: git_info::StdGitCommitLinkCow,
    msg: NotFoundMsg,
    open_api_specification: OpenApiSpecificationPath,
}
#[derive(Debug, serde::Serialize, optml::Optml, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(max = NOT_FOUND_MSG_MAX_LEN, description = "not found message")]
#[newtype(display)]
struct NotFoundMsg(String);
#[derive(Debug, Clone, Copy, serde::Serialize, optml::Optml)]
struct OpenApiSpecificationPath(&'static str);
#[derive(Debug, Clone, Copy, optml::Optml)]
struct AxumHttpUriRef<'uri_lt>(&'uri_lt axum::http::Uri);
#[derive(Debug, Clone, Copy, optml::Optml)]
struct UriSuffixRef<'suffix_lt>(&'suffix_lt str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
struct NoRouteMsgCapacity(usize);
#[derive(Debug, Clone, Copy, optml::Optml)]
struct HealthCheckSucceeded(bool);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HealthDatabaseAvailable(bool);
impl From<bool> for HealthDatabaseAvailable {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Degraded,
    Error,
    Ok,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum HealthComponentKind {
    DatabaseConnectivity,
    ServiceAvailability,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
pub struct HealthComponent {
    kind: HealthComponentKind,
    status: HealthStatus,
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
pub struct HealthReport {
    components: Vec<HealthComponent>,
    status: HealthStatus,
}
impl HealthReport {
    #[must_use]
    pub fn liveness() -> Self {
        Self {
            components: vec![HealthComponent {
                kind: HealthComponentKind::ServiceAvailability,
                status: HealthStatus::Ok,
            }],
            status: HealthStatus::Ok,
        }
    }
    #[must_use]
    pub fn readiness(database_available: HealthDatabaseAvailable) -> Self {
        let database_status = if database_available.0 {
            HealthStatus::Ok
        } else {
            HealthStatus::Error
        };
        let status = if database_available.0 {
            HealthStatus::Ok
        } else {
            HealthStatus::Degraded
        };
        Self {
            components: vec![
                HealthComponent {
                    kind: HealthComponentKind::ServiceAvailability,
                    status: HealthStatus::Ok,
                },
                HealthComponent {
                    kind: HealthComponentKind::DatabaseConnectivity,
                    status: database_status,
                },
            ],
            status,
        }
    }
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
struct AxumHealthCheckStatus(axum::http::StatusCode);
#[derive(Debug, optml::Optml)]
struct JsonRes<T> {
    payload: AxumJsonPayload<T>,
    status: AxumHealthCheckStatus,
}
#[derive(Debug, optml::Optml)]
struct AxumJsonPayload<T>(axum::Json<T>);
impl<T> axum::response::IntoResponse for AxumJsonPayload<T>
where
    axum::Json<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
impl<T> axum::response::IntoResponse for JsonRes<T>
where
    AxumJsonPayload<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        (self.status.0, self.payload).into_response()
    }
}
#[derive(Debug, Clone, optml::Optml, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct AxumCmnRoutes(axum::Router);
#[derive(Clone, optml::Optml)]
pub struct StdArcCmnRoutesAppState(std::sync::Arc<dyn CmnRoutesPrms>);
#[derive(Clone, Copy, Debug, utoipa::OpenApi)]
#[openapi(
    paths(health_live, git_info_open_api),
    components(schemas(
        HealthReport,
        HealthComponent,
        HealthComponentKind,
        HealthStatus,
        GitInfo
    ))
)]
pub struct CmnRoutesOpenApi;
#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct UtoipaCmnRoutesOpenApiDocument(utoipa::openapi::OpenApi);
impl CmnRoutesOpenApi {
    #[must_use]
    pub fn open_api() -> UtoipaCmnRoutesOpenApiDocument {
        UtoipaCmnRoutesOpenApiDocument(<Self as utoipa::OpenApi>::openapi())
    }
}
impl std::fmt::Debug for StdArcCmnRoutesAppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StdArcCmnRoutesAppState").finish()
    }
}
impl std::fmt::Debug for UtoipaCmnRoutesOpenApiDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UtoipaCmnRoutesOpenApiDocument").finish()
    }
}
impl<AppStateTy> From<std::sync::Arc<AppStateTy>> for StdArcCmnRoutesAppState
where
    AppStateTy: CmnRoutesPrms + 'static,
{
    fn from(value: std::sync::Arc<AppStateTy>) -> Self {
        Self(value)
    }
}
pub trait CmnRoutesPrms:
    git_info::GetGitCommitLink + app_state::GetSqlxPgPool + Send + Sync
{
}
#[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between handlers and tests
const fn mk_git_info_payload(commit: git_info::StdGitCommitLinkCow) -> GitInfo {
    GitInfo { commit }
}
#[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests
fn mk_no_route_msg(uri: AxumHttpUriRef<'_>) -> NotFoundMsg {
    mk_no_route_msg_for_suffix(get_uri_suffix(uri))
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed
fn mk_no_route_msg_for_suffix(uri_suffix: UriSuffixRef<'_>) -> NotFoundMsg {
    let cap = no_route_msg_capacity(uri_suffix);
    let mut msg = String::with_capacity(cap.0);
    msg.push_str(NO_ROUTE_MSG_PREFIX);
    msg.push_str(uri_suffix.0);
    NotFoundMsg::try_from(msg).unwrap_or_else(NotFoundMsg::from)
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and message builder
const fn no_route_msg_capacity(uri_suffix: UriSuffixRef<'_>) -> NoRouteMsgCapacity {
    NoRouteMsgCapacity(NO_ROUTE_MSG_PREFIX.len().saturating_add(uri_suffix.0.len()))
}
#[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs
fn get_uri_suffix(uri: AxumHttpUriRef<'_>) -> UriSuffixRef<'_> {
    UriSuffixRef(
        uri.0
            .path_and_query()
            .map_or_else(|| uri.0.path(), |v| v.as_str()),
    )
}
#[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place
fn mk_not_found_payload(
    uri: AxumHttpUriRef<'_>,
    commit: git_info::StdGitCommitLinkCow,
) -> NotFoundH {
    mk_not_found_payload_with_msg(mk_no_route_msg(uri), commit)
}
#[allow(clippy::single_call_fn)] // shared payload constructor keeps not-found response shape centralized
const fn mk_not_found_payload_with_msg(
    msg: NotFoundMsg,
    commit: git_info::StdGitCommitLinkCow,
) -> NotFoundH {
    NotFoundH {
        commit,
        msg,
        open_api_specification: OpenApiSpecificationPath(SLASH_SWAGGER_UI),
    }
}
#[allow(clippy::single_call_fn)] // shared helper keeps commit-based status+json responses consistent across handlers
fn mk_commit_json_res<S, T>(
    commit_src: &S,
    status: AxumHealthCheckStatus,
    map: impl FnOnce(git_info::StdGitCommitLinkCow) -> T,
) -> JsonRes<T>
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
const fn mk_json_res<T>(status: AxumHealthCheckStatus, payload: T) -> JsonRes<T> {
    JsonRes {
        status,
        payload: AxumJsonPayload(axum::Json(payload)),
    }
}
#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
const fn map_health_check_status(is_ok: HealthCheckSucceeded) -> AxumHealthCheckStatus {
    if is_ok.0 {
        HEALTH_CHECK_OK_STATUS
    } else {
        HEALTH_CHECK_ER_STATUS
    }
}
async fn database_is_ready(app_state: &dyn CmnRoutesPrms) -> HealthCheckSucceeded {
    let pool = app_state::GetSqlxPgPool::get_sqlx_pg_pool(app_state);
    let probe = async {
        sqlx::query(HEALTH_CHECK_SQL)
            .execute(pool.as_ref())
            .await
            .is_ok()
    };
    HealthCheckSucceeded(bool::from(
        server_runtime::run_health_probe(
            server_runtime::StdHealthProbeTimeout::from(HEALTH_PROBE_TIMEOUT),
            probe,
        )
        .await,
    ))
}
const fn health_report_response(report: HealthReport) -> JsonRes<HealthReport> {
    let status = match report.status() {
        HealthStatus::Ok => HEALTH_CHECK_OK_STATUS,
        HealthStatus::Degraded | HealthStatus::Error => HEALTH_CHECK_ER_STATUS,
    };
    mk_json_res(status, report)
}
#[utoipa::path(get, path = "/health/live", responses((status = 200, body = HealthReport)), tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally shared by Axum and OpenAPI metadata"
)]
async fn health_live() -> JsonRes<HealthReport> {
    health_report_response(HealthReport::liveness())
}
#[utoipa::path(get, path = "/git_info", responses((status = 200, body = GitInfo)), tag = "service")]
#[allow(
    dead_code,
    clippy::single_call_fn,
    reason = "Utoipa consumes this metadata-only handler through its derive expansion"
)]
const fn git_info_open_api() {}
#[must_use]
pub fn cmn_routes(app_state_b9fc2d94: StdArcCmnRoutesAppState) -> AxumCmnRoutes {
    let app_state = app_state_b9fc2d94.0;
    AxumCmnRoutes(
        axum::Router::new()
            .route(SLASH_HEALTH_LIVE, axum::routing::get(health_live))
            .route(
                SLASH_HEALTH_READY,
                axum::routing::get(async |axum::extract::State(app_state_raw)| {
                    let ready_state: std::sync::Arc<dyn CmnRoutesPrms> = app_state_raw;
                    health_report_response(HealthReport::readiness(HealthDatabaseAvailable::from(
                        database_is_ready(ready_state.as_ref()).await.0,
                    )))
                }),
            )
            .route(
                SLASH_HEALTH,
                axum::routing::get(async |axum::extract::State(app_state_raw)| {
                    let aggregate_state: std::sync::Arc<dyn CmnRoutesPrms> = app_state_raw;
                    health_report_response(HealthReport::readiness(HealthDatabaseAvailable::from(
                        database_is_ready(aggregate_state.as_ref()).await.0,
                    )))
                }),
            )
            .route(
                SLASH_HEALTH_CHECK,
                axum::routing::get(async |axum::extract::State(app_state_hc_raw)| {
                    let app_state_hc: std::sync::Arc<dyn CmnRoutesPrms> = app_state_hc_raw;
                    map_health_check_status(database_is_ready(app_state_hc.as_ref()).await).0
                }),
            )
            .route(
                SLASH_GIT_INFO,
                axum::routing::get(async |axum::extract::State(app_state_raw)| {
                    let app_state_76fb2013: std::sync::Arc<dyn CmnRoutesPrms> = app_state_raw;
                    mk_commit_json_res(
                        app_state_76fb2013.as_ref(),
                        AxumHealthCheckStatus(axum::http::StatusCode::OK),
                        mk_git_info_payload,
                    )
                }),
            )
            .fallback(async |uri, axum::extract::State(app_state_19103bd5_raw)| {
                let app_state_19103bd5: std::sync::Arc<dyn CmnRoutesPrms> = app_state_19103bd5_raw;
                mk_commit_json_res(
                    app_state_19103bd5.as_ref(),
                    AxumHealthCheckStatus(axum::http::StatusCode::NOT_FOUND),
                    |commit| mk_not_found_payload(AxumHttpUriRef(&uri), commit),
                )
            })
            .with_state(app_state),
    )
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)] // fixtures remain adjacent to the tests that exercise their route state
mod tests {
    const TEST_COMMIT: &str = "abc123";
    #[test]
    fn health_reports_distinguish_liveness_and_dependency_readiness() {
        let live = super::HealthReport::liveness();
        assert_eq!(live.status(), super::HealthStatus::Ok);
        assert_eq!(live.components.len(), 1usize);
        let ready = super::HealthReport::readiness(super::HealthDatabaseAvailable::from(true));
        assert_eq!(ready.status(), super::HealthStatus::Ok);
        assert_eq!(ready.components.len(), 2usize);
        let degraded = super::HealthReport::readiness(super::HealthDatabaseAvailable::from(false));
        assert_eq!(degraded.status(), super::HealthStatus::Degraded);
        assert_eq!(
            degraded.components.get(1usize).expect("16ca1c84").status,
            super::HealthStatus::Error
        );
    }
    #[derive(Debug)]
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
    impl super::CmnRoutesPrms for TestState {}
    fn test_state() -> std::sync::Arc<dyn super::CmnRoutesPrms> {
        std::sync::Arc::new(TestState {
            commit: TEST_COMMIT,
        })
    }
    fn test_commit_link() -> String {
        git_info::git_commit_link(TEST_COMMIT).as_ref().to_owned()
    }
    #[allow(clippy::single_call_fn)] // shared owned->Cow conversion keeps commit-link payload setup consistent across tests
    fn test_commit_link_cow() -> git_info::StdGitCommitLinkCow {
        git_info::StdGitCommitLinkCow::from(std::borrow::Cow::Owned(test_commit_link()))
    }
    fn b_cow(v: &'static str) -> git_info::StdGitCommitLinkCow {
        git_info::StdGitCommitLinkCow::from(std::borrow::Cow::Borrowed(v))
    }
    const fn uri_ref(uri: &axum::http::Uri) -> super::AxumHttpUriRef<'_> {
        super::AxumHttpUriRef(uri)
    }
    const fn suffix_ref(v: &str) -> super::UriSuffixRef<'_> {
        super::UriSuffixRef(v)
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps git-info payload checks concise and consistent
    fn assert_git_info_commit(payload: &super::GitInfo, exp_commit: &str) {
        assert_eq!(payload.commit.as_ref(), exp_commit);
    }
    #[allow(clippy::single_call_fn)] // shared assertion centralizes not-found payload checks used across direct and state-based tests
    fn assert_not_found_payload(payload: &super::NotFoundH, exp_uri_suffix: &str) {
        assert_no_route_msg(&payload.msg, exp_uri_suffix);
        assert_eq!(payload.open_api_specification.0, super::SLASH_SWAGGER_UI);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps not-found commit and payload checks coupled across tests
    fn assert_not_found_payload_with_commit(
        payload: &super::NotFoundH,
        exp_commit: &str,
        exp_uri_suffix: &str,
    ) {
        assert_eq!(payload.commit.as_ref(), exp_commit);
        assert_not_found_payload(payload, exp_uri_suffix);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps no-route message checks consistent across uri and suffix-based tests
    fn assert_no_route_msg(actual: &super::NotFoundMsg, uri_suffix: &str) {
        assert_eq!(
            actual.0,
            super::mk_no_route_msg_for_suffix(suffix_ref(uri_suffix)).0
        );
    }
    #[test]
    fn git_info_response_shape_stays_stable() {
        let git_info = super::mk_git_info_payload(b_cow("abc123"));
        assert_git_info_commit(&git_info, "abc123");
    }
    #[test]
    fn not_found_response_shape_stays_stable() {
        let uri = axum::http::Uri::from_static("/unknown");
        let not_found = super::mk_not_found_payload(uri_ref(&uri), b_cow("deadbeef"));
        assert_not_found_payload_with_commit(&not_found, "deadbeef", "/unknown");
    }
    #[test]
    fn no_route_msg_includes_uri() {
        let uri = axum::http::Uri::from_static("/missing/path");
        assert_no_route_msg(&super::mk_no_route_msg(uri_ref(&uri)), "/missing/path");
    }
    #[test]
    fn no_route_msg_for_suffix_uses_prefix_once() {
        assert_no_route_msg(
            &super::mk_no_route_msg_for_suffix(suffix_ref("/missing/path")),
            "/missing/path",
        );
    }
    #[test]
    fn get_uri_suffix_prefers_path_and_query_when_query_exists() {
        let uri = axum::http::Uri::from_static("/missing/path?limit=10");
        assert_eq!(
            super::get_uri_suffix(uri_ref(&uri)).0,
            "/missing/path?limit=10"
        );
    }
    #[test]
    fn no_route_msg_keeps_query_parameters() {
        let uri = axum::http::Uri::from_static("/missing/path?limit=10");
        assert_no_route_msg(
            &super::mk_no_route_msg(uri_ref(&uri)),
            "/missing/path?limit=10",
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
        let payload = super::mk_git_info_payload(
            git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
        );
        assert_git_info_commit(&payload, test_commit_link().as_str());
    }
    #[test]
    fn not_found_response_uses_uri_and_swagger_path() {
        let uri = axum::http::Uri::from_static("/missing");
        let commit_link = test_commit_link();
        let payload = super::mk_not_found_payload(uri_ref(&uri), test_commit_link_cow());
        assert_not_found_payload_with_commit(&payload, &commit_link, "/missing");
    }
    #[test]
    fn not_found_payload_from_state_uses_uri_and_swagger_path() {
        let uri = axum::http::Uri::from_static("/missing");
        let state = test_state();
        let payload = super::mk_not_found_payload(
            uri_ref(&uri),
            git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
        );
        assert_not_found_payload_with_commit(&payload, &test_commit_link(), "/missing");
    }
    #[test]
    fn not_found_payload_for_suffix_uses_given_suffix_and_swagger_path() {
        let commit_link = test_commit_link();
        let payload = super::mk_not_found_payload_with_msg(
            super::mk_no_route_msg_for_suffix(suffix_ref("/missing")),
            test_commit_link_cow(),
        );
        assert_not_found_payload_with_commit(&payload, &commit_link, "/missing");
    }
    #[test]
    fn no_route_prefix_stays_stable() {
        assert_eq!(super::NO_ROUTE_MSG_PREFIX, "No route for ");
    }
    #[test]
    fn no_route_msg_capacity_is_exact_for_uri_suffix() {
        assert_eq!(
            super::no_route_msg_capacity(suffix_ref("/abc?x=1")).0,
            "No route for /abc?x=1".len()
        );
    }
    #[test]
    fn map_health_check_status_returns_ok_for_success() {
        assert_eq!(
            super::map_health_check_status(super::HealthCheckSucceeded(true)),
            super::HEALTH_CHECK_OK_STATUS
        );
    }
    #[test]
    fn map_health_check_status_returns_unavailable_for_error() {
        assert_eq!(
            super::map_health_check_status(super::HealthCheckSucceeded(false)),
            super::HEALTH_CHECK_ER_STATUS
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
    fn mk_json_res_wraps_payload_with_status() {
        let response = super::mk_json_res(
            super::AxumHealthCheckStatus(axum::http::StatusCode::CREATED),
            super::mk_git_info_payload(b_cow(TEST_COMMIT)),
        );
        assert_eq!(response.status.0, axum::http::StatusCode::CREATED);
        assert_git_info_commit(&response.payload.0, TEST_COMMIT);
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
        let response = super::mk_commit_json_res(
            test_state().as_ref(),
            super::AxumHealthCheckStatus(axum::http::StatusCode::OK),
            super::mk_git_info_payload,
        );
        assert_eq!(response.status.0, axum::http::StatusCode::OK);
        assert_git_info_commit(&response.payload.0, test_commit_link().as_str());
    }
    #[tokio::test]
    async fn runtime_health_version_and_public_read_match_openapi() {
        let router = axum::Router::from(super::cmn_routes(super::StdArcCmnRoutesAppState(
            test_state(),
        )));
        let document = serde_json::to_value(super::CmnRoutesOpenApi::open_api()).expect("f96bcc6e");
        let check = |path: &'static str| {
            let cloned_router = router.clone();
            let cloned_document = document.clone();
            async move {
                let response = tower::ServiceExt::oneshot(
                    cloned_router,
                    axum::http::Request::builder()
                        .uri(path)
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
                let escaped_path = path.replace('/', "~1");
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
        check("/health/live").await;
        check("/git_info").await;
        let not_found = tower::ServiceExt::oneshot(
            router,
            axum::http::Request::builder()
                .uri("/missing")
                .body(axum::body::Body::empty())
                .expect("bb258755"),
        )
        .await
        .expect("d2b9cc45");
        assert_eq!(not_found.status(), axum::http::StatusCode::NOT_FOUND);
    }
}
