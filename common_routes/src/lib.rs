#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "generated route registries stay adjacent to their handlers and utoipa expands to an internal for_each"
)]
const HEALTH_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2u64);
const HEALTH_COMPONENTS_MAX_LEN: usize = 2usize;
#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, optml::Optml)]
pub struct GitInfo {
    #[schema(value_type = String)]
    commit: git_info::StdGitCommitLinkCow,
}
#[derive(Debug, serde::Serialize, optml::Optml)]
struct NotFoundHandle {
    commit: git_info::StdGitCommitLinkCow,
    message: to_err_string::ErrorText,
    open_api_specification: OpenApiSpecificationPath,
}
#[derive(Debug, Clone, Copy, serde::Serialize, optml::Optml, newtype::FromInner)]
struct OpenApiSpecificationPath(&'static str);
#[derive(Debug, Clone, Copy, optml::Optml, newtype::FromInner)]
struct AxumHttpUriRef<'uri_lt>(&'uri_lt axum::http::Uri);
#[derive(Debug, Clone, Copy, optml::Optml, newtype::FromInner)]
struct UriSuffixRef<'suffix_lt>(&'suffix_lt str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner)]
struct NoRouteMessageCapacity(usize);
#[derive(Debug, Clone, Copy, optml::Optml, newtype::FromInner)]
struct HealthCheckSucceeded(bool);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct HealthDatabaseAvailable(bool);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Degraded,
    Error,
    Ok,
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthComponentKind {
    DatabaseConnectivity,
    ServiceAvailability,
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
pub struct HealthComponent {
    kind: HealthComponentKind,
    status: HealthStatus,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::TryFrom,
)]
#[try_from(validator = HealthComponents::validate)]
#[serde(try_from = "Vec<HealthComponent>")]
pub struct HealthComponents(Vec<HealthComponent>);
impl From<[HealthComponent; 1]> for HealthComponents {
    fn from(value: [HealthComponent; 1]) -> Self {
        Self(Vec::from(value))
    }
}
impl From<[HealthComponent; 2]> for HealthComponents {
    fn from(value: [HealthComponent; 2]) -> Self {
        Self(Vec::from(value))
    }
}
impl HealthComponents {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &[HealthComponent]) -> Result<(), HealthComponentsError> {
        if value.len() > HEALTH_COMPONENTS_MAX_LEN {
            Err(HealthComponentsError)
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", str_constants::HEALTH_COMPONENTS_LENGTH_EXCEEDS_LIMIT)]
pub struct HealthComponentsError;
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct HealthReport {
    components: HealthComponents,
    status: HealthStatus,
}
impl HealthReport {
    #[must_use]
    pub fn liveness() -> Self {
        Self {
            components: HealthComponents::from([HealthComponent {
                kind: HealthComponentKind::ServiceAvailability,
                status: HealthStatus::Ok,
            }]),
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
            components: HealthComponents::from([
                HealthComponent {
                    kind: HealthComponentKind::ServiceAvailability,
                    status: HealthStatus::Ok,
                },
                HealthComponent {
                    kind: HealthComponentKind::DatabaseConnectivity,
                    status: database_status,
                },
            ]),
            status,
        }
    }
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner)]
struct AxumHealthCheckStatus(axum::http::StatusCode);
impl axum::response::IntoResponse for AxumHealthCheckStatus {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
#[derive(Debug, optml::Optml)]
struct JsonRes<T> {
    payload: AxumJsonPayload<T>,
    status: AxumHealthCheckStatus,
}
#[derive(Debug, optml::Optml, newtype::FromInner)]
struct AxumJsonPayload<T>(axum::Json<T>);
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CommonNoBody;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_statuses = &[],
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_live",
    path = "/health/live",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport
)]
pub struct HealthLiveRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_response = HealthReport,
    error_statuses = &[frontend_contract::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_ready",
    path = "/health/ready",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport
)]
pub struct HealthReadyRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_response = HealthReport,
    error_statuses = &[frontend_contract::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health",
    path = "/health",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport
)]
pub struct HealthRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_response = (),
    error_statuses = &[frontend_contract::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_check",
    path = "/health_check",
    request = CommonNoBody,
    response = CommonNoBody,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport
)]
pub struct HealthCheckRoute;

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_statuses = &[],
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "git_info",
    path = "/git_info",
    request = CommonNoBody,
    response = GitInfo,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport
)]
pub struct GitInfoRoute;

#[derive(Clone, Copy, Debug, Eq, PartialEq, frontend_contract::RouteCatalog)]
#[route_catalog(family = CommonRouteFamily, body_limit = 0usize)]
pub enum CommonRoute {
    #[route_catalog_route(GitInfoRoute)]
    GitInfo,
    #[route_catalog_route(HealthRoute)]
    Health,
    #[route_catalog_route(HealthCheckRoute)]
    HealthCheck,
    #[route_catalog_route(HealthLiveRoute)]
    HealthLive,
    #[route_catalog_route(HealthReadyRoute)]
    HealthReady,
}
impl CommonRoute {
    #[must_use]
    pub fn path(self) -> frontend_contract::ContractStr {
        self.contract().path()
    }
}
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
#[derive(Debug, Clone, optml::Optml, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct AxumCommonRoutes(axum::Router);
#[derive(Clone, optml::Optml, newtype::FromInner)]
pub struct StdArcCommonRoutesAppState(std::sync::Arc<dyn CommonRoutesParameters>);
#[derive(Clone, Copy, Debug)]
pub struct CommonRoutesOpenApi;
#[derive(serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner, newtype::IntoInnerFrom)]
pub struct UtoipaCommonRoutesOpenApiDocument(utoipa::openapi::OpenApi);
impl CommonRoutesOpenApi {
    #[must_use]
    pub fn open_api() -> UtoipaCommonRoutesOpenApiDocument {
        UtoipaCommonRoutesOpenApiDocument::from(CommonRouteRegistry::open_api())
    }
}
impl std::fmt::Debug for StdArcCommonRoutesAppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(str_constants::STDARCCOMMONROUTESAPPSTATE)
            .finish()
    }
}
impl axum::extract::FromRequestParts<Self> for StdArcCommonRoutesAppState {
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &Self,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(state.clone()))
    }
}
impl std::fmt::Debug for UtoipaCommonRoutesOpenApiDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(str_constants::UTOIPACOMMONROUTESOPENAPIDOCUMENT)
            .finish()
    }
}
impl<AppStateTy> From<std::sync::Arc<AppStateTy>> for StdArcCommonRoutesAppState
where
    AppStateTy: CommonRoutesParameters + 'static,
{
    fn from(value: std::sync::Arc<AppStateTy>) -> Self {
        Self(value)
    }
}

pub trait CommonRoutesParameters:
    git_info::GetGitCommitLink + app_state::GetSqlxPgPool + Send + Sync
{
}
fn health_check_ok_status() -> AxumHealthCheckStatus {
    AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
}
fn health_check_er_status() -> AxumHealthCheckStatus {
    AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
}
#[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between handlers and tests
const fn mk_git_info_payload(commit: git_info::StdGitCommitLinkCow) -> GitInfo {
    GitInfo { commit }
}
#[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests
fn mk_no_route_message(uri: AxumHttpUriRef<'_>) -> to_err_string::ErrorText {
    mk_no_route_message_for_suffix(get_uri_suffix(uri))
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed
fn mk_no_route_message_for_suffix(uri_suffix: UriSuffixRef<'_>) -> to_err_string::ErrorText {
    let cap = no_route_message_capacity(uri_suffix);
    let mut message = String::with_capacity(cap.0);
    message.push_str(str_constants::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX);
    message.push_str(uri_suffix.0);
    to_err_string::ErrorText::try_from(message).unwrap_or_else(to_err_string::ErrorText::from)
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and message builder
fn no_route_message_capacity(uri_suffix: UriSuffixRef<'_>) -> NoRouteMessageCapacity {
    NoRouteMessageCapacity::from(
        str_constants::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
            .len()
            .saturating_add(uri_suffix.0.len()),
    )
}
#[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs
fn get_uri_suffix(uri: AxumHttpUriRef<'_>) -> UriSuffixRef<'_> {
    UriSuffixRef::from(
        uri.0
            .path_and_query()
            .map_or_else(|| uri.0.path(), |v| v.as_str()),
    )
}
#[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place
fn mk_not_found_payload(
    uri: AxumHttpUriRef<'_>,
    commit: git_info::StdGitCommitLinkCow,
) -> NotFoundHandle {
    mk_not_found_payload_with_message(mk_no_route_message(uri), commit)
}
#[allow(clippy::single_call_fn)] // shared payload constructor keeps not-found response shape centralized
fn mk_not_found_payload_with_message(
    message: to_err_string::ErrorText,
    commit: git_info::StdGitCommitLinkCow,
) -> NotFoundHandle {
    NotFoundHandle {
        commit,
        message,
        open_api_specification: OpenApiSpecificationPath::from(
            str_constants::COMMON_ROUTES_SWAGGER_UI,
        ),
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
fn mk_json_res<T>(status: AxumHealthCheckStatus, payload: T) -> JsonRes<T> {
    JsonRes {
        status,
        payload: AxumJsonPayload::from(axum::Json(payload)),
    }
}
#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
fn map_health_check_status(is_ok: HealthCheckSucceeded) -> AxumHealthCheckStatus {
    if is_ok.0 {
        health_check_ok_status()
    } else {
        health_check_er_status()
    }
}
async fn database_is_ready(app_state: &dyn CommonRoutesParameters) -> HealthCheckSucceeded {
    let pool = app_state::GetSqlxPgPool::get_sqlx_pg_pool(app_state);
    let probe = async {
        sqlx::query(str_constants::COMMON_ROUTES_HEALTH_CHECK_SQL)
            .execute(pool.as_ref())
            .await
            .is_ok()
    };
    HealthCheckSucceeded::from(bool::from(
        server_runtime::run_health_probe(
            server_runtime::StdHealthProbeTimeout::from(HEALTH_PROBE_TIMEOUT),
            probe,
        )
        .await,
    ))
}
fn health_report_response(report: HealthReport) -> JsonRes<HealthReport> {
    let status = match report.status() {
        HealthStatus::Ok => health_check_ok_status(),
        HealthStatus::Degraded | HealthStatus::Error => health_check_er_status(),
    };
    mk_json_res(status, report)
}
#[frontend_contract::route_registry(
    state = StdArcCommonRoutesAppState,
    family = CommonRouteFamily;
    ("", "");
    schemas(
        HealthComponent,
        HealthComponentKind,
        HealthComponents,
        HealthStatus
    );
    (GitInfoRoute, git_info),
    (HealthRoute, health),
    (HealthCheckRoute, health_check),
    (HealthLiveRoute, health_live),
    (HealthReadyRoute, health_ready),
)]
#[openapi(tags((name = "service", description = "Service operational routes")))]
struct CommonRouteRegistry;

#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally shared by Axum and OpenAPI metadata"
)]
async fn health_live() -> JsonRes<HealthReport> {
    health_report_response(HealthReport::liveness())
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn health_ready(app_state: StdArcCommonRoutesAppState) -> JsonRes<HealthReport> {
    health_report_response(HealthReport::readiness(HealthDatabaseAvailable::from(
        database_is_ready(app_state.0.as_ref()).await.0,
    )))
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn health(app_state: StdArcCommonRoutesAppState) -> JsonRes<HealthReport> {
    health_report_response(HealthReport::readiness(HealthDatabaseAvailable::from(
        database_is_ready(app_state.0.as_ref()).await.0,
    )))
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn health_check(app_state: StdArcCommonRoutesAppState) -> AxumHealthCheckStatus {
    map_health_check_status(database_is_ready(app_state.0.as_ref()).await)
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn git_info(app_state: StdArcCommonRoutesAppState) -> JsonRes<GitInfo> {
    mk_commit_json_res(
        app_state.0.as_ref(),
        AxumHealthCheckStatus::from(axum::http::StatusCode::OK),
        mk_git_info_payload,
    )
}

#[must_use]
pub fn common_routes(app_state_b9fc2d94: StdArcCommonRoutesAppState) -> AxumCommonRoutes {
    AxumCommonRoutes::from(
        CommonRouteRegistry::router()
            .fallback(async |uri, axum::extract::State(app_state_19103bd5_raw)| {
                let app_state_19103bd5: StdArcCommonRoutesAppState = app_state_19103bd5_raw;
                mk_commit_json_res(
                    app_state_19103bd5.0.as_ref(),
                    AxumHealthCheckStatus::from(axum::http::StatusCode::NOT_FOUND),
                    |commit| mk_not_found_payload(AxumHttpUriRef::from(&uri), commit),
                )
            })
            .with_state(app_state_b9fc2d94),
    )
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)] // fixtures remain adjacent to the tests that exercise their route state
mod tests {
    #[test]
    fn repository_owned_common_routes_use_snake_case_segments() {
        assert!(!str_constants::COMMON_ROUTES_SWAGGER_UI.contains('-'));
        super::CommonRoute::ALL.into_iter().for_each(|route| {
            assert!(!route.path().as_ref().contains('-'));
        });
    }

    #[test]
    fn common_route_family_coverage_is_complete() {
        let descriptors =
            <super::CommonRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
        assert_eq!(
            frontend_contract::validate_route_coverage(descriptors.as_ref()),
            Ok(())
        );
        assert_eq!(descriptors.as_ref().len(), super::CommonRoute::ALL.len());
    }

    #[test]
    fn health_reports_distinguish_liveness_and_dependency_readiness() {
        let live = super::HealthReport::liveness();
        assert_eq!(live.status(), super::HealthStatus::Ok);
        assert_eq!(live.components.0.len(), 1usize);
        let ready = super::HealthReport::readiness(super::HealthDatabaseAvailable::from(true));
        assert_eq!(ready.status(), super::HealthStatus::Ok);
        assert_eq!(ready.components.0.len(), 2usize);
        let degraded = super::HealthReport::readiness(super::HealthDatabaseAvailable::from(false));
        assert_eq!(degraded.status(), super::HealthStatus::Degraded);
        assert_eq!(
            degraded.components.0.get(1usize).expect("16ca1c84").status,
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
    #[allow(clippy::single_call_fn)] // shared owned->Cow conversion keeps commit-link payload setup consistent across tests
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
    #[allow(clippy::single_call_fn)] // shared assertion keeps git-info payload checks concise and consistent
    fn assert_git_info_commit(payload: &super::GitInfo, exp_commit: &str) {
        assert_eq!(payload.commit.as_ref(), exp_commit);
    }
    #[allow(clippy::single_call_fn)] // shared assertion centralizes not-found payload checks used across direct and state-based tests
    fn assert_not_found_payload(payload: &super::NotFoundHandle, exp_uri_suffix: &str) {
        assert_no_route_message(&payload.message, exp_uri_suffix);
        assert_eq!(
            payload.open_api_specification.0,
            str_constants::COMMON_ROUTES_SWAGGER_UI
        );
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps not-found commit and payload checks coupled across tests
    fn assert_not_found_payload_with_commit(
        payload: &super::NotFoundHandle,
        exp_commit: &str,
        exp_uri_suffix: &str,
    ) {
        assert_eq!(payload.commit.as_ref(), exp_commit);
        assert_not_found_payload(payload, exp_uri_suffix);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps no-route message checks consistent across uri and suffix-based tests
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
    fn health_components_rejects_more_than_supported_components() {
        let component = super::HealthComponent {
            kind: super::HealthComponentKind::ServiceAvailability,
            status: super::HealthStatus::Ok,
        };
        assert_eq!(
            super::HealthComponents::try_from(vec![component, component, component]),
            Err(super::HealthComponentsError)
        );
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
        let payload = super::mk_git_info_payload(
            git_info::GetGitCommitLink::get_git_commit_link_cow(state.as_ref()),
        );
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
    fn map_health_check_status_returns_ok_for_success() {
        assert_eq!(
            super::map_health_check_status(super::HealthCheckSucceeded(true)),
            super::health_check_ok_status()
        );
    }
    #[test]
    fn map_health_check_status_returns_unavailable_for_error() {
        assert_eq!(
            super::map_health_check_status(super::HealthCheckSucceeded(false)),
            super::health_check_er_status()
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
            super::AxumHealthCheckStatus::from(axum::http::StatusCode::CREATED),
            super::mk_git_info_payload(b_cow(str_constants::TEST_VALUES_COMMIT)),
        );
        assert_eq!(response.status.0, axum::http::StatusCode::CREATED);
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
        let response = super::mk_commit_json_res(
            test_state().as_ref(),
            super::AxumHealthCheckStatus::from(axum::http::StatusCode::OK),
            super::mk_git_info_payload,
        );
        assert_eq!(response.status.0, axum::http::StatusCode::OK);
        assert_git_info_commit(&response.payload.0, test_commit_link().as_str());
    }
    #[tokio::test]
    async fn runtime_health_version_and_public_read_match_openapi() {
        let router = axum::Router::from(super::common_routes(
            super::StdArcCommonRoutesAppState::from(test_state()),
        ));
        let document =
            serde_json::to_value(super::CommonRoutesOpenApi::open_api()).expect("f96bcc6e");
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
}
