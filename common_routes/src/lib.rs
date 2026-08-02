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
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct HealthDatabaseAvailable(bool);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Degraded,
    Error,
    Ok,
}
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthComponentKind {
    DatabaseConnectivity,
    ServiceAvailability,
}
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct HealthComponent {
    kind: HealthComponentKind,
    status: HealthStatus,
}
#[derive(optml::Optml, Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct HealthComponents(Vec<HealthComponent>);
impl utoipa::PartialSchema for HealthComponents {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::BoundedVec<
            HealthComponent,
            0usize,
            HEALTH_COMPONENTS_MAX_LEN,
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for HealthComponents {}
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
impl TryFrom<Vec<HealthComponent>> for HealthComponents {
    type Error = HealthComponentsError;

    fn try_from(value: Vec<HealthComponent>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::<HealthComponent, 0usize, HEALTH_COMPONENTS_MAX_LEN>::try_from(
            value,
        )
        .map(bounded_types::BoundedVec::into_inner)
        .map(Self)
        .map_err(|_error| HealthComponentsError)
    }
}
impl<'de> serde::Deserialize<'de> for HealthComponents {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <bounded_types::BoundedVec<
            HealthComponent,
            0usize,
            HEALTH_COMPONENTS_MAX_LEN,
        > as serde::Deserialize>::deserialize(deserializer)?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", str_constants::HEALTH_COMPONENTS_LENGTH_EXCEEDS_LIMIT)]
pub struct HealthComponentsError;
#[derive(
    optml::Optml,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
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
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum CommonNotFoundError {
    #[error("common route was not found")]
    NotFound(NotFoundHandle),
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum HealthCheckError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum HealthError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum HealthLiveError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(optml::Optml, Debug, thiserror::Error)]
enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(Debug, optml::Optml, newtype::FromInner)]
struct AxumJsonPayload<T>(axum::Json<T>);
#[derive(
    optml::Optml, Clone, Copy, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
pub struct CommonNoBody;

#[derive(optml::Optml, Clone, Copy, Debug, frontend_contract::TypedRoute)]
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

#[derive(optml::Optml, Clone, Copy, Debug, frontend_contract::TypedRoute)]
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

#[derive(optml::Optml, Clone, Copy, Debug, frontend_contract::TypedRoute)]
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

#[derive(optml::Optml, Clone, Copy, Debug, frontend_contract::TypedRoute)]
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

#[derive(optml::Optml, Clone, Copy, Debug, frontend_contract::TypedRoute)]
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

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, frontend_contract::RouteCatalog)]
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
        self.payload.into_response()
    }
}
impl axum::response::IntoResponse for CommonNotFoundError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound(payload) => axum::response::IntoResponse::into_response((
                axum::http::StatusCode::NOT_FOUND,
                axum::Json(payload),
            )),
        }
    }
}
impl axum::response::IntoResponse for HealthCheckError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable => axum::response::IntoResponse::into_response(
                frontend_contract::ApiProblemError::ServiceUnavailable,
            ),
        }
    }
}
impl axum::response::IntoResponse for HealthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable => health_unavailable_response(),
        }
    }
}
impl axum::response::IntoResponse for HealthLiveError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable => health_unavailable_response(),
        }
    }
}
impl axum::response::IntoResponse for HealthReadyError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable => health_unavailable_response(),
        }
    }
}
fn health_unavailable_response() -> axum::response::Response {
    axum::response::IntoResponse::into_response(
        frontend_contract::ApiProblemError::ServiceUnavailable,
    )
}
#[derive(Debug, Clone, optml::Optml, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct AxumCommonRoutes(axum::Router);
#[derive(Clone, optml::Optml, newtype::FromInner)]
pub struct StdArcCommonRoutesAppState(std::sync::Arc<dyn CommonRoutesParameters>);
#[derive(optml::Optml, Clone, Copy, Debug)]
pub struct CommonRoutesOpenApi;
#[derive(optml::Optml, serde::Serialize)]
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
    map: impl FnOnce(git_info::StdGitCommitLinkCow) -> T,
) -> JsonRes<T>
where
    S: ?Sized + git_info::GetGitCommitLink,
{
    mk_json_res(map(git_info::GetGitCommitLink::get_git_commit_link_cow(
        commit_src,
    )))
}
fn mk_json_res<T>(payload: T) -> JsonRes<T> {
    JsonRes {
        payload: AxumJsonPayload::from(axum::Json(payload)),
    }
}
#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
fn map_health_check_status(is_ok: HealthCheckSucceeded) -> AxumHealthCheckStatus {
    if is_ok.0 {
        AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
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
        server_runtime_http::run_health_probe(
            server_runtime_http::StdHealthProbeTimeout::from(HEALTH_PROBE_TIMEOUT),
            probe,
        )
        .await,
    ))
}
fn health_report_response(report: HealthReport) -> Option<JsonRes<HealthReport>> {
    match report.status() {
        HealthStatus::Ok => Some(mk_json_res(report)),
        HealthStatus::Degraded | HealthStatus::Error => None,
    }
}
#[derive(optml::Optml)]
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
async fn health_live() -> Result<JsonRes<HealthReport>, HealthLiveError> {
    health_report_response(HealthReport::liveness()).ok_or(HealthLiveError::Unavailable)
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn health_ready(
    app_state: StdArcCommonRoutesAppState,
) -> Result<JsonRes<HealthReport>, HealthReadyError> {
    health_report_response(HealthReport::readiness(HealthDatabaseAvailable::from(
        database_is_ready(app_state.0.as_ref()).await.0,
    )))
    .ok_or(HealthReadyError::Unavailable)
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn health(
    app_state: StdArcCommonRoutesAppState,
) -> Result<JsonRes<HealthReport>, HealthError> {
    health_report_response(HealthReport::readiness(HealthDatabaseAvailable::from(
        database_is_ready(app_state.0.as_ref()).await.0,
    )))
    .ok_or(HealthError::Unavailable)
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn health_check(
    app_state: StdArcCommonRoutesAppState,
) -> Result<AxumHealthCheckStatus, HealthCheckError> {
    let status = map_health_check_status(database_is_ready(app_state.0.as_ref()).await);
    if status.0 == axum::http::StatusCode::OK {
        Ok(status)
    } else {
        Err(HealthCheckError::Unavailable)
    }
}
#[frontend_contract::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete handler is intentionally owned by the generated route registry"
)]
async fn git_info(app_state: StdArcCommonRoutesAppState) -> JsonRes<GitInfo> {
    mk_commit_json_res(app_state.0.as_ref(), mk_git_info_payload)
}

#[must_use]
pub fn common_routes(app_state_b9fc2d94: StdArcCommonRoutesAppState) -> AxumCommonRoutes {
    AxumCommonRoutes::from(
        CommonRouteRegistry::router()
            .fallback(async |uri, axum::extract::State(app_state_19103bd5_raw)| {
                let app_state_19103bd5: StdArcCommonRoutesAppState = app_state_19103bd5_raw;
                CommonNotFoundError::NotFound(mk_not_found_payload(
                    AxumHttpUriRef::from(&uri),
                    git_info::GetGitCommitLink::get_git_commit_link_cow(
                        app_state_19103bd5.0.as_ref(),
                    ),
                ))
            })
            .with_state(app_state_b9fc2d94),
    )
}
#[cfg(test)]
mod tests;
