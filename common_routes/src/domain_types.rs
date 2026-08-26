#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "generated route registries stay adjacent to their endpoints and utoipa expands to an internal for_each"
)]
pub(crate) const HEALTH_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2u64);
const HEALTH_COMPONENTS_MAX_LEN: usize = 2usize;
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct GitInfo {
    #[schema(value_type = String)]
    commit: git_info::domain_types::GitCommitLinkCow,
}
#[derive(Debug, serde::Serialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct NotFoundPayload {
    commit: git_info::domain_types::GitCommitLinkCow,
    message: to_err_string::domain_types::ErrorText,
    open_api_specification: OpenApiSpecificationPath,
}
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
struct OpenApiSpecificationPath(&'static str);
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct AxumHttpUriRef<'uri_lt>(&'uri_lt axum::http::Uri);
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct UriSuffixRef<'suffix_lt>(&'suffix_lt str);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
struct NoRouteMessageCapacity(usize);
#[derive(
    Debug,
    Clone,
    Copy,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct HealthCheckSucceeded(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HealthDatabaseAvailable(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, serde::Serialize,
)]
pub struct HealthComponents(Vec<HealthComponent>);
impl utoipa::PartialSchema for HealthComponents {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            HealthComponent,
            { constants_usize::ZERO },
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
        bounded_types::domain_types::vector::BoundedVec::<
            HealthComponent,
            { constants_usize::ZERO },
            HEALTH_COMPONENTS_MAX_LEN,
        >::try_from(value)
        .map(bounded_types::domain_types::vector::BoundedVec::into_inner)
        .map(Self)
        .map_err(|_error| HealthComponentsError)
    }
}
impl<'de> serde::Deserialize<'de> for HealthComponents {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <bounded_types::domain_types::vector::BoundedVec<
            HealthComponent,
            { constants_usize::ZERO },
            HEALTH_COMPONENTS_MAX_LEN,
        > as serde::Deserialize>::deserialize(deserializer)?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::HEALTH_COMPONENTS_LENGTH_EXCEEDS_LIMIT)]
pub struct HealthComponentsError;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
pub(crate) struct AxumHealthCheckStatus(axum::http::StatusCode);
impl AxumHealthCheckStatus {
    pub(crate) fn is_ok(self) -> HealthCheckSucceeded {
        HealthCheckSucceeded::from(self.0 == axum::http::StatusCode::OK)
    }
}
impl axum::response::IntoResponse for AxumHealthCheckStatus {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct JsonRes<T> {
    payload: AxumJsonPayload<T>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum CommonNotFoundError {
    #[error("common route was not found")]
    NotFound(NotFoundPayload),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthCheckError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthLiveError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable,
}
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct AxumJsonPayload<T>(axum::Json<T>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct CommonNoBody;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_statuses = &[],
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_live",
    path = "/health/live",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct HealthLiveRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_response = HealthReport,
    error_statuses = &[frontend_contract::domain_types::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_ready",
    path = "/health/ready",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct HealthReadyRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_response = HealthReport,
    error_statuses = &[frontend_contract::domain_types::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health",
    path = "/health",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct HealthRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_response = (),
    error_statuses = &[frontend_contract::domain_types::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_check",
    path = "/health_check",
    request = CommonNoBody,
    response = CommonNoBody,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct HealthCheckRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_statuses = &[],
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "git_info",
    path = "/git_info",
    request = CommonNoBody,
    response = GitInfo,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct GitInfoRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::domain_types::RouteCatalog,
)]
#[route_catalog(family = CommonRouteFamily, body_limit = constants_usize::ZERO)]
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
    pub fn path(self) -> frontend_contract::domain_types::ContractStr {
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
                frontend_contract::domain_types::ApiProblemError::ServiceUnavailable,
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
        frontend_contract::domain_types::ApiProblemError::ServiceUnavailable,
    )
}
#[derive(
    Debug,
    Clone,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct AxumCommonRoutes(axum::Router);
#[derive(Clone, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub struct ArcCommonRoutesAppState(std::sync::Arc<dyn CommonRoutesParameters>);
impl ArcCommonRoutesAppState {
    pub(crate) fn get(&self) -> &dyn CommonRoutesParameters {
        self.0.as_ref()
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct CommonRoutesOpenApi;
#[derive(optimal_memory_layout::OptimalMemoryLayout, serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner, newtype::IntoInnerFrom)]
pub struct UtoipaCommonRoutesOpenApiDocument(utoipa::openapi::OpenApi);
impl CommonRoutesOpenApi {
    #[must_use]
    pub fn open_api() -> UtoipaCommonRoutesOpenApiDocument {
        crate::adapters::open_api()
    }
}
impl std::fmt::Debug for ArcCommonRoutesAppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::STDARCCOMMONROUTESAPPSTATE)
            .finish()
    }
}
impl axum::extract::FromRequestParts<Self> for ArcCommonRoutesAppState {
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
        f.debug_tuple(constants_str::UTOIPACOMMONROUTESOPENAPIDOCUMENT)
            .finish()
    }
}
impl<AppStateTy> From<std::sync::Arc<AppStateTy>> for ArcCommonRoutesAppState
where
    AppStateTy: CommonRoutesParameters + 'static,
{
    fn from(value: std::sync::Arc<AppStateTy>) -> Self {
        Self(value)
    }
}

pub trait CommonRoutesParameters:
    git_info::domain_types::GitCommitLinkProvider
    + app_state::domain_types::SqlxPgPoolProvider
    + Send
    + Sync
{
}
#[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between endpoints and tests
pub(crate) const fn make_git_info_payload(
    commit: git_info::domain_types::GitCommitLinkCow,
) -> GitInfo {
    GitInfo { commit }
}
#[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests
fn make_no_route_message(uri: AxumHttpUriRef<'_>) -> to_err_string::domain_types::ErrorText {
    make_no_route_message_for_suffix(uri_suffix(uri))
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed
fn make_no_route_message_for_suffix(
    uri_suffix: UriSuffixRef<'_>,
) -> to_err_string::domain_types::ErrorText {
    let cap = NoRouteMessageCapacity::from(
        constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
            .len()
            .saturating_add(uri_suffix.0.len()),
    );
    let mut message = String::with_capacity(cap.0);
    message.push_str(constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX);
    message.push_str(uri_suffix.0);
    to_err_string::domain_types::ErrorText::try_from(message)
        .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
}
#[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs
fn uri_suffix(uri: AxumHttpUriRef<'_>) -> UriSuffixRef<'_> {
    UriSuffixRef::from(
        uri.0
            .path_and_query()
            .map_or_else(|| uri.0.path(), |v| v.as_str()),
    )
}
#[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place
pub(crate) fn make_not_found_payload(
    uri: AxumHttpUriRef<'_>,
    commit: git_info::domain_types::GitCommitLinkCow,
) -> NotFoundPayload {
    make_not_found_payload_with_message(make_no_route_message(uri), commit)
}
#[allow(clippy::single_call_fn)] // shared payload constructor keeps not-found response shape centralized
fn make_not_found_payload_with_message(
    message: to_err_string::domain_types::ErrorText,
    commit: git_info::domain_types::GitCommitLinkCow,
) -> NotFoundPayload {
    NotFoundPayload {
        commit,
        message,
        open_api_specification: OpenApiSpecificationPath::from(
            constants_str::COMMON_ROUTES_SWAGGER_UI,
        ),
    }
}
#[allow(clippy::single_call_fn)] // shared helper keeps commit-based status+json responses consistent across endpoints
pub(crate) fn make_commit_json_response<S, T>(
    commit_src: &S,
    map: impl FnOnce(git_info::domain_types::GitCommitLinkCow) -> T,
) -> JsonRes<T>
where
    S: ?Sized + git_info::domain_types::GitCommitLinkProvider,
{
    make_json_response(map(
        git_info::domain_types::GitCommitLinkProvider::git_commit_link_cow(commit_src),
    ))
}
pub(crate) fn make_json_response<T>(payload: T) -> JsonRes<T> {
    JsonRes {
        payload: AxumJsonPayload::from(axum::Json(payload)),
    }
}
#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
pub(crate) fn map_health_check_status(is_ok: HealthCheckSucceeded) -> AxumHealthCheckStatus {
    if is_ok.0 {
        AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    }
}
#[cfg(test)]
#[path = "domain_types_tests.rs"]
mod tests;
