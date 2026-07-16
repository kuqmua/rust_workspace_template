mod background_job;
mod batched_cleanup;
mod bounded_read;
mod child_process;
mod client_ip;
mod cors;
mod csp;
mod deduplicating_queue;
mod exclusive_run;
mod fallback;
mod generation_gate;
mod geojson;
mod header_text;
mod health;
mod history;
mod http_header_policy;
mod http_policy;
mod http_status_error;
mod lease_registry;
mod lifecycle;
mod limits;
mod metrics_layer;
mod multipart;
mod notification;
mod origin;
mod outbound_url;
mod password_policy;
mod path_policy;
mod pg_rate_limit;
mod redacted_url;
mod request_id;
mod resource_budget;
mod resource_utilization;
mod retry;
mod secret_text;
mod secure_cookie;
mod service_bootstrap;
mod single_flight;
mod text_policy;
mod wire_token;
pub use background_job::BackgroundJob;
pub use batched_cleanup::{
    CleanupBatchCount, CleanupBatchSize, CleanupBatchSizeError, CleanupCompletion,
    CleanupContinuation, CleanupReport, CleanupRows, run_batched_cleanup,
};
pub use bounded_read::{
    BoundedBytes, BoundedJsonReadError, BoundedJsonText, BoundedReadError, BoundedReadMaximumBytes,
    BoundedText, IoErrorPresenceDisposition, ReqwestError, ReqwestResponse, SerdeJsonError,
    StdBoundedReadConcurrency, StdBoundedReadConcurrencyMaximum, StdFromUtf8Error, StdIoError,
    StdPathRef, classify_not_found_io_error, parse_bounded_json, read_bounded_file,
    read_bounded_file_async, read_bounded_http_response, read_bounded_json_file_async,
    read_bounded_json_http_response,
};
pub use child_process::{
    ChildDiagnostic, ChildProcessCompletion, ChildProcessError, ChildProcessId, ChildProcessReport,
    ChildProcessReports, ChildProcessSet, ChildProcessSetError, ChildProcessSucceeded,
    ChildProcessSupervisor, StdChildDiagnosticMaximum, StdChildExitStatus, StdChildProcessIoError,
    StdChildProcessSetMaximum, TokioChildProcess, TokioChildProcessJoinError,
};
pub use client_ip::{
    HttpHeaderMapRef, StdAddrParseError, StdParseIntError, StdResolvedClientIp, StdSocketAddr,
    TrustedProxyRange, TrustedProxyRangeParseError, TrustedProxyRanges, resolve_client_ip,
    resolve_header_text,
};
pub use cors::{
    HttpCorsAllowOriginHeaderValues, HttpCorsAllowOriginTextRef, parse_cors_allow_origin,
};
pub use csp::{
    HttpCspBuilder, HttpCspDirectiveName, HttpCspDirectiveValue, HttpCspMaximumBytesError,
    HttpCspTokenError,
};
pub use deduplicating_queue::{DeduplicatingQueue, QueuePush, StdQueueMaximum};
pub use exclusive_run::{ExclusiveRun, ExclusiveRunAlreadyActive, ExclusiveRunGuard};
pub use fallback::{
    FallbackResponseMode, HttpAcceptHeaderMaximumBytes, HttpFallbackApiPrefixRef,
    HttpFallbackMetricsPathRef, HttpFallbackRequestPathRef, HttpOptionalAcceptHeaderRef,
    fallback_response_mode,
};
pub use generation_gate::{Generation, GenerationCommit, GenerationGate};
pub use geojson::{GeoJsonDocumentText, GeoJsonValidationError, SerdeJsonGeoJsonError};
pub use header_text::{
    HttpHeaderName, HttpHeaderTextBytes, HttpHeaderTextMaximumBytes,
    HttpHeaderTextMaximumBytesError, HttpHeaderTextRef, HttpHeaderTextResolution,
};
pub use health::{
    HealthComponentStatus, HealthProbeSucceeded, HealthReadiness, HealthSnapshot,
    ServiceLivenessSnapshot, StdHealthProbeTimeout, add_health_routes, run_health_probe,
};
pub use history::{
    AsyncRunHistory, AsyncRunHistorySnapshot, StdAsyncRunHistoryMaximumLen,
    StdAsyncRunHistoryMaximumLenTryFromUsizeError, StdAsyncRunHistoryReportCount,
};
pub use http_header_policy::{
    HttpAttachmentFileNameRef, HttpContentDisposition, HttpContentDispositionError,
    HttpContentLength, HttpContentLengthError, build_attachment_content_disposition,
};
pub use http_policy::{
    BearerAuthorizationResolution, CookieResolution, HttpAuthorizationHeaderTextRef,
    HttpBearerTokenRef, HttpContentTypeTextRef, HttpCookieHeadersRef, HttpCookieNameRef,
    HttpCookieValueRef, OptionalJsonBodyPresence, OptionalJsonContentType,
    OptionalJsonContentTypeDecision, classify_optional_json_content_type,
    optional_json_content_type_decision, resolve_bearer_authorization, resolve_unique_cookie,
};
pub use http_status_error::{HttpErrorClass, HttpErrorStatus, classify_http_error_status};
pub use lease_registry::{
    LeaseHeartbeat, LeaseId, LeaseIds, LeaseKey, LeaseRegistry, LeaseReservation, LeaseState,
    LeaseTextError, StdLeaseRegistryMaximum, StdLeaseStaleTimeout, StdLeaseStaleTimeoutError,
};
pub use lifecycle::{
    BackgroundTask, BackgroundTaskOutcome, BackgroundTaskShutdownError, StdRequestTimeout,
    StdRequestTimeoutTryFromDurationError, StdRunInterval, StdRunIntervalTryFromDurationError,
    TokioAbortTask, TokioTaskJoinError, abort_and_wait_task, spawn_interval_task,
};
pub use limits::{
    AcquirePermitError, RetryAfterSecs, RetryAfterSecsTryFromU64Error, StdArcTokioSemaphore,
    StdPermitWaitTimeout, StdSemaphorePermitCount, TokioAcquireError, TokioOwnedSemaphorePermit,
    acquire_permit,
};
pub use metrics_layer::{
    HttpMetricsLayer, HttpMetricsPathCacheMaximum, HttpMetricsPathCacheMaximumTryFromUsizeError,
    MetricsResponseBody, MetricsResponseBodyError,
};
pub use multipart::{
    FileStagingAction, FileStagingDirectoryName, MultipartBytes, MultipartBytesPart,
    MultipartFieldName, MultipartFileName, MultipartPayloadMaximum, MultipartRequestError,
    MultipartTextPart, MultipartTextValue, MultipartUploadRequest, MultipartValueError,
    MultipartValueLength, StdStorageRelativePath, StoragePathSegment, StoragePathSegmentError,
    identifier_file_storage_relative_path, staging_directory_name,
};
pub use notification::{
    AxumNotificationRouter, NotificationApiToken, NotificationApiTokenAuthorized,
    NotificationApiTokenError, NotificationApiTokenRef, NotificationMessage,
    NotificationMessageError, NotificationRequest, NotificationSender, NotificationServiceState,
    notification_router,
};
pub use origin::{
    AllowedOrigin, AllowedOriginError, AllowedOrigins, AllowedOriginsError, HttpOriginHeadersRef,
    RequestOriginAllowed, request_origin_allowed,
};
pub use outbound_url::{
    OutboundAllowedHost, OutboundHostAllowlist, OutboundHostAllowlistError, OutboundHostPolicy,
    OutboundUrlError, OutboundUrlPolicy, OutboundUrlScheme, OutboundUrlTextRef, ReqwestOutboundUrl,
    StdOutboundIpAddr,
};
pub use password_policy::{
    PasswordLength, PasswordLengthRange, PasswordLengthRangeError, PasswordPolicyViolation,
    PasswordTextRef, validate_password_policy,
};
pub use path_policy::{
    HttpAllowedPathPrefixRef, HttpNormalizedPath, HttpNormalizedPathError, HttpProxyPath,
    HttpProxyPathError, HttpProxyPathPrefixMatch, HttpProxyPathRef, HttpRequestPathRef,
    normalize_identifier_path, proxy_path_matches_prefix,
};
pub use pg_rate_limit::{
    PgRateLimitDecision, PgRateLimitError, PgRateLimitMaximum, PgRateLimitQueryRef,
    PgRateLimitScopeRef, PgRateLimitSubjectRef, PgRateLimitValidationError,
    PgRateLimitWindowSeconds, SqlxPgRateLimitError, SqlxPgRateLimitPoolRef, enforce_pg_rate_limit,
};
pub use redacted_url::{
    RedactedUrl, RedactedUrlTextRef, redact_rtsp_url_userinfo, redact_url_userinfo,
};
pub use request_id::{
    HttpHeaderToStrError, RequestId, RequestIdTryFromHttpHeaderValueError,
    RequestIdTryFromStringError,
};
pub use resource_budget::{
    GetBulkItemResourceBudget, GetIdempotencyResponseResourceBudget, ResourceBudget,
    ResourceBudgetAmount, ResourceBudgetConfigError, ResourceBudgetMaximum,
    ResourceBudgetReservation, ResourceBudgetReserveError,
};
pub use resource_utilization::{
    ResourceAmount, ResourceUtilization, ResourceUtilizationError, ResourceUtilizationPercent,
    ResourceUtilizationStatus, calculate_resource_utilization,
};
pub use retry::{
    RetryOutcome, RetryPolicy, StdRetryAttempts, StdRetryAttemptsError, StdRetryDelay,
    run_with_retries,
};
pub use secret_text::{
    BoundedSecretText, BoundedSecretTextError, SecretTextMatch, SecretTextRef, secret_texts_match,
};
pub use secure_cookie::{
    HttpCookieAccess, HttpCookieName, HttpCookieSecure, HttpCookieValue, HttpSecureCookieError,
    HttpSetCookieHeaderValue, StdCookieMaxAgeSeconds, build_secure_strict_cookie,
};
pub use service_bootstrap::{
    ServiceTracingFormat, StdServiceRuntimeIoError, TokioServiceRuntime,
    TracingSubscriberInitError, build_service_runtime, initialize_service_tracing,
    wait_for_service_shutdown_signal,
};
pub use single_flight::{
    SingleFlight, SingleFlightAcquire, SingleFlightKey, SingleFlightKeyError, SingleFlightOwner,
    SingleFlightWaitOutcome, SingleFlightWaiter, StdSingleFlightMaximum,
};
pub use text_policy::{
    BoundedTextPolicyError, FixedLengthAsciiHexText, FixedLengthAsciiHexTextError,
    NonEmptyTrimmedText, RequiredNulFreeBoundedText, UrlSafeTokenPartText,
    UrlSafeTokenPartTextError,
};
pub use wire_token::{VersionedUrlSafeWireTokenText, VersionedUrlSafeWireTokenTextError};
#[derive(Debug)]
pub struct AxumRouter(axum::Router);
impl From<axum::Router> for AxumRouter {
    fn from(value: axum::Router) -> Self {
        Self(value)
    }
}
impl From<AxumRouter> for axum::Router {
    fn from(value: AxumRouter) -> Self {
        value.0
    }
}
#[derive(Clone, Debug)]
pub struct ReqwestClient(reqwest::Client);
impl Default for ReqwestClient {
    fn default() -> Self {
        Self(reqwest::Client::new())
    }
}
impl From<ReqwestClient> for reqwest::Client {
    fn from(value: ReqwestClient) -> Self {
        value.0
    }
}
#[derive(Debug)]
pub struct ServiceRuntime {
    optional_task: Option<BackgroundTask>,
    router: AxumRouter,
}
impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(self) -> (AxumRouter, Option<BackgroundTask>) {
        (self.router, self.optional_task)
    }
    #[must_use]
    pub const fn new(router: AxumRouter, optional_task: Option<BackgroundTask>) -> Self {
        Self {
            optional_task,
            router,
        }
    }
}
#[derive(Debug)]
pub struct TokioTcpListener(tokio::net::TcpListener);
impl From<tokio::net::TcpListener> for TokioTcpListener {
    fn from(value: tokio::net::TcpListener) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct RequestIdLayer;
impl RequestIdLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter(router.0.layer(RequestIdTowerLayer))
    }
}
#[derive(Clone, Copy, Debug)]
struct RequestIdTowerLayer;
#[derive(Clone, Debug)]
struct RequestIdService<Service> {
    inner: Service,
}
impl<Service> tower::Layer<Service> for RequestIdTowerLayer {
    type Service = RequestIdService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        RequestIdService { inner }
    }
}
impl<Service> tower::Service<axum::extract::Request> for RequestIdService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let request_id_and_header_value = [
            str_constants::HTTP_HEADER_NAMES_X_REQUEST_ID,
            str_constants::RUNTIME_CORRELATION_ID_HEADER_NAME,
        ]
        .into_iter()
        .find_map(|header_name| {
            req.headers().get(header_name).and_then(|value| {
                RequestId::try_from(value)
                    .ok()
                    .map(|request_id| (request_id, value.clone()))
            })
        })
        .unwrap_or_else(|| {
            loop {
                if let Ok(value) = RequestId::try_from(uuid::Uuid::new_v4().to_string())
                    && let Ok(header_value) = http::HeaderValue::try_from(&value)
                {
                    break (value, header_value);
                }
            }
        });
        let started_at = tokio::time::Instant::now();
        let span = tracing::info_span!("http.request", request_id = %request_id_and_header_value.0, method = %req.method(), path = %req.uri().path());
        let _previous_extension_request_id =
            req.extensions_mut().insert(request_id_and_header_value.0);
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(tracing::Instrument::instrument(
            async move {
                let mut response = response_future.await?;
                tracing::info!(
                    status = response.status().as_u16(),
                    duration_ms = started_at.elapsed().as_millis(),
                    "http request completed"
                );
                let _previous_header_request_id = response.headers_mut().insert(
                    http::HeaderName::from_static(str_constants::HTTP_HEADER_NAMES_X_REQUEST_ID),
                    request_id_and_header_value.1.clone(),
                );
                let _previous_correlation_id = response.headers_mut().insert(
                    http::HeaderName::from_static(
                        str_constants::RUNTIME_CORRELATION_ID_HEADER_NAME,
                    ),
                    request_id_and_header_value.1,
                );
                Ok(response)
            },
            span,
        ))
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct RequestTimeoutLayer(StdRequestTimeout);
impl From<StdRequestTimeout> for RequestTimeoutLayer {
    fn from(value: StdRequestTimeout) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(transparent)]
struct StdRequestTimeoutMessage(&'static str);
#[derive(Debug, serde::Serialize)]
struct RequestTimeoutBody {
    error: StdRequestTimeoutMessage,
}
impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter(router.0.layer(RequestTimeoutTowerLayer(self.0)))
    }
}
#[derive(Clone, Copy, Debug)]
struct RequestTimeoutTowerLayer(StdRequestTimeout);
#[derive(Clone, Debug)]
struct RequestTimeoutService<Service> {
    inner: Service,
    timeout: StdRequestTimeout,
}
impl<Service> tower::Layer<Service> for RequestTimeoutTowerLayer {
    type Service = RequestTimeoutService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        RequestTimeoutService {
            inner,
            timeout: self.0,
        }
    }
}
impl<Service> tower::Service<axum::extract::Request> for RequestTimeoutService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, req: axum::extract::Request) -> Self::Future {
        let response_future = tower::Service::call(&mut self.inner, req);
        let timeout = self.timeout;
        Box::pin(async move {
            match tokio::time::timeout(timeout.get(), response_future).await {
                Ok(response) => response,
                Err(_elapsed) => {
                    let retry_after = timeout.get().as_secs().max(1u64).to_string();
                    let mut response = axum::response::IntoResponse::into_response((
                        http::StatusCode::SERVICE_UNAVAILABLE,
                        axum::Json(RequestTimeoutBody {
                            error: StdRequestTimeoutMessage(str_constants::REQUEST_TIMEOUT),
                        }),
                    ));
                    if let Ok(value) = http::HeaderValue::from_str(retry_after.as_str()) {
                        let _previous = response
                            .headers_mut()
                            .insert(http::header::RETRY_AFTER, value);
                    }
                    Ok(response)
                }
            }
        })
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForwardedProtoTrust {
    Ignore,
    Trust,
}
#[derive(Clone, Debug)]
pub struct HttpContentSecurityPolicy(http::HeaderValue);
#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("content security policy is not a valid HTTP header value")]
pub struct HttpContentSecurityPolicyError;
impl TryFrom<String> for HttpContentSecurityPolicy {
    type Error = HttpContentSecurityPolicyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4096usize {
            return Err(HttpContentSecurityPolicyError);
        }
        http::HeaderValue::try_from(value)
            .map(Self)
            .map_err(|_error| HttpContentSecurityPolicyError)
    }
}
#[derive(Clone, Debug)]
pub struct SecurityHeadersLayer {
    content_security_policy: Option<HttpContentSecurityPolicy>,
    forwarded_proto_trust: ForwardedProtoTrust,
}
impl From<ForwardedProtoTrust> for SecurityHeadersLayer {
    fn from(value: ForwardedProtoTrust) -> Self {
        Self {
            content_security_policy: None,
            forwarded_proto_trust: value,
        }
    }
}
impl SecurityHeadersLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter(router.0.layer(SecurityHeadersTowerLayer {
            content_security_policy: self.content_security_policy,
            forwarded_proto_trust: self.forwarded_proto_trust,
        }))
    }
    #[must_use]
    pub fn with_content_security_policy(mut self, value: HttpContentSecurityPolicy) -> Self {
        self.content_security_policy = Some(value);
        self
    }
}
#[derive(Clone, Debug)]
struct SecurityHeadersTowerLayer {
    content_security_policy: Option<HttpContentSecurityPolicy>,
    forwarded_proto_trust: ForwardedProtoTrust,
}
#[derive(Clone, Debug)]
struct SecurityHeadersService<Service> {
    content_security_policy: Option<HttpContentSecurityPolicy>,
    forwarded_proto_trust: ForwardedProtoTrust,
    inner: Service,
}
impl<Service> tower::Layer<Service> for SecurityHeadersTowerLayer {
    type Service = SecurityHeadersService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        SecurityHeadersService {
            content_security_policy: self.content_security_policy.clone(),
            forwarded_proto_trust: self.forwarded_proto_trust,
            inner,
        }
    }
}
impl<Service> tower::Service<axum::extract::Request> for SecurityHeadersService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let is_api_path = req.uri().path().starts_with(str_constants::API);
        let is_forwarded_https = matches!(self.forwarded_proto_trust, ForwardedProtoTrust::Trust)
            && req
                .headers()
                .get(str_constants::X_FORWARDED_PROTO)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    value.split(',').next().is_some_and(|first| {
                        first.trim().eq_ignore_ascii_case(str_constants::HTTPS)
                    })
                });
        req.headers_mut().iter_mut().for_each(|(name, value)| {
            if name == http::header::AUTHORIZATION
                || name == http::header::COOKIE
                || name.as_str() == str_constants::X_CSRF_TOKEN_ALT
            {
                value.set_sensitive(true);
            }
        });
        let content_security_policy = self.content_security_policy.clone();
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(async move {
            let mut response = response_future.await?;
            let _content_type_options = response.headers_mut().insert(
                http::HeaderName::from_static(str_constants::X_CONTENT_TYPE_OPTIONS),
                http::HeaderValue::from_static(str_constants::NOSNIFF),
            );
            let _frame_options = response.headers_mut().insert(
                http::HeaderName::from_static(str_constants::X_FRAME_OPTIONS),
                http::HeaderValue::from_static(str_constants::DENY),
            );
            let _referrer_policy = response.headers_mut().insert(
                http::HeaderName::from_static(str_constants::REFERRER_POLICY),
                http::HeaderValue::from_static(str_constants::NO_REFERRER),
            );
            if let Some(resolved_content_security_policy) = content_security_policy {
                let _previous_content_security_policy = response.headers_mut().insert(
                    http::HeaderName::from_static(str_constants::CONTENT_SECURITY_POLICY_HEADER),
                    resolved_content_security_policy.0,
                );
            }
            response.headers_mut().iter_mut().for_each(|(name, value)| {
                if name == http::header::SET_COOKIE {
                    value.set_sensitive(true);
                }
            });
            if is_api_path {
                let _cache_control = response.headers_mut().insert(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_static(str_constants::NO_STORE),
                );
            }
            if is_forwarded_https {
                let _strict_transport_security = response.headers_mut().insert(
                    http::HeaderName::from_static(str_constants::STRICT_TRANSPORT_SECURITY),
                    http::HeaderValue::from_static(
                        str_constants::MAX_AGE_31536000_INCLUDESUBDOMAINS,
                    ),
                );
            }
            Ok(response)
        })
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
#[derive(Debug)]
pub struct StdServeIoError(std::io::Error);
impl std::fmt::Display for StdServeIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdServeIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum ServeWithGracefulShutdownError {
    Serve(StdServeIoError),
    ShutdownTimeout,
}
impl std::fmt::Display for ServeWithGracefulShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serve(error) => write!(f, "server failed: {error}"),
            Self::ShutdownTimeout => f.write_str(str_constants::SERVER_GRACEFUL_SHUTDOWN_TIMED_OUT),
        }
    }
}
impl std::error::Error for ServeWithGracefulShutdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serve(error) => Some(error),
            Self::ShutdownTimeout => None,
        }
    }
}
#[must_use]
pub fn add_status_route(router: AxumRouter) -> AxumRouter {
    AxumRouter(router.0.route(
        str_constants::STATUS,
        axum::routing::get(async || http::StatusCode::OK),
    ))
}
#[allow(clippy::integer_division_remainder_used)] // tokio::select expansion uses internal randomized branch arithmetic
pub async fn serve_with_graceful_shutdown<Shutdown>(
    listener: TokioTcpListener,
    router: AxumRouter,
    shutdown: Shutdown,
    shutdown_timeout: StdRequestTimeout,
) -> Result<(), ServeWithGracefulShutdownError>
where
    Shutdown: Future<Output = ()> + Send + 'static,
{
    let (shutdown_started_tx, shutdown_started_rx) = tokio::sync::oneshot::channel();
    let server = IntoFuture::into_future(
        axum::serve(
            listener.0,
            router
                .0
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .with_graceful_shutdown(async move {
            shutdown.await;
            let _send_result = shutdown_started_tx.send(());
        }),
    );
    tokio::pin!(server);
    tokio::select! {
        result = &mut server => result.map_err(|error| ServeWithGracefulShutdownError::Serve(StdServeIoError(error))),
        shutdown_result = shutdown_started_rx => {
            drop(shutdown_result);
            tokio::time::timeout(shutdown_timeout.get(), &mut server)
                .await
                .map_err(|_elapsed| ServeWithGracefulShutdownError::ShutdownTimeout)?
                .map_err(|error| ServeWithGracefulShutdownError::Serve(StdServeIoError(error)))
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn resource_budget_reservations_are_bounded_and_released() {
        let budget = super::ResourceBudget::new(
            super::ResourceBudgetMaximum::try_from(5usize).expect("0c6362a4"),
        );
        let first = budget
            .reserve(super::ResourceBudgetAmount::from(3usize))
            .expect("3bfeb37c");
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(3usize));
        assert_eq!(
            budget
                .reserve(super::ResourceBudgetAmount::from(3usize))
                .expect_err("3c31187b"),
            super::ResourceBudgetReserveError::Exhausted
        );
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(3usize));
        let second = budget
            .reserve(super::ResourceBudgetAmount::from(2usize))
            .expect("d86085db");
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(5usize));
        drop(first);
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(2usize));
        drop(second);
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(0usize));
    }
    #[test]
    fn resource_budget_reports_overflow_without_changing_count() {
        let budget = super::ResourceBudget::new(
            super::ResourceBudgetMaximum::try_from(usize::MAX).expect("65f2f229"),
        );
        let reservation = budget
            .reserve(super::ResourceBudgetAmount::from(1usize))
            .expect("1a2bb321");
        assert_eq!(
            budget
                .reserve(super::ResourceBudgetAmount::from(usize::MAX))
                .expect_err("e317c775"),
            super::ResourceBudgetReserveError::Overflow
        );
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(1usize));
        drop(reservation);
    }
    #[test]
    fn concurrent_resource_budget_reservations_never_exceed_maximum() {
        let budget = super::ResourceBudget::new(
            super::ResourceBudgetMaximum::try_from(5usize).expect("57a61ca4"),
        );
        let start = std::sync::Arc::new(std::sync::Barrier::new(3usize));
        let finish = std::sync::Arc::new(std::sync::Barrier::new(3usize));
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::scope(|scope| {
            let left_budget = budget.clone();
            let left_start = std::sync::Arc::clone(&start);
            let left_finish = std::sync::Arc::clone(&finish);
            let left_tx = tx.clone();
            let _left_handle = scope.spawn(move || {
                let _start_result = left_start.wait();
                let reservation = left_budget.reserve(super::ResourceBudgetAmount::from(3usize));
                left_tx.send(reservation.is_ok()).expect("b048535e");
                let _finish_result = left_finish.wait();
                drop(reservation);
            });
            let right_budget = budget.clone();
            let right_start = std::sync::Arc::clone(&start);
            let right_finish = std::sync::Arc::clone(&finish);
            let right_tx = tx.clone();
            let _right_handle = scope.spawn(move || {
                let _start_result = right_start.wait();
                let reservation = right_budget.reserve(super::ResourceBudgetAmount::from(3usize));
                right_tx.send(reservation.is_ok()).expect("cd734995");
                let _finish_result = right_finish.wait();
                drop(reservation);
            });
            let _start_result = start.wait();
            let outcomes = [rx.recv().expect("7393afca"), rx.recv().expect("67824b65")];
            assert_eq!(outcomes.into_iter().filter(|value| *value).count(), 1usize);
            assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(3usize));
            let _finish_result = finish.wait();
        });
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(0usize));
    }
    #[tokio::test]
    async fn async_run_history_keeps_latest_reports() {
        let history = super::AsyncRunHistory::new(
            super::StdAsyncRunHistoryMaximumLen::try_from(2usize).expect("8567a9df"),
        );
        history.push(1u8).await;
        history.push(2u8).await;
        history.push(3u8).await;
        let snapshot = history.snapshot().await;
        assert_eq!(usize::from(snapshot.report_count()), 2usize);
        assert_eq!(snapshot.latest_report(), Some(&3u8));
    }
    #[tokio::test]
    async fn service_runtime_status_route_and_parts_are_stable() {
        let runtime = super::ServiceRuntime::new(
            super::add_status_route(super::AxumRouter::from(axum::Router::new())),
            None,
        );
        let (router, optional_task) = runtime.into_parts();
        assert!(optional_task.is_none());
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(router),
            axum::extract::Request::builder()
                .uri(str_constants::STATUS)
                .body(axum::body::Body::empty())
                .expect("8e9c3da1"),
        )
        .await
        .expect("1e97ad3b");
        assert_eq!(response.status(), http::StatusCode::OK);
        let optional_interval_task = super::spawn_interval_task(None, async || {});
        assert!(optional_interval_task.is_none());
    }
    #[tokio::test]
    async fn background_task_shutdown_is_observable() {
        let interval = super::StdRunInterval::try_from(std::time::Duration::from_secs(1u64))
            .expect("e76640c4");
        let task = super::spawn_interval_task(Some(interval), async || {}).expect("32858863");
        let timeout = super::StdRequestTimeout::try_from(std::time::Duration::from_secs(1u64))
            .expect("728b52b3");
        assert_eq!(
            task.shutdown(timeout).await.expect("0d71d1b8"),
            super::BackgroundTaskOutcome::ShutdownRequested
        );
    }
    #[tokio::test]
    async fn background_task_panic_is_observable() {
        let interval = super::StdRunInterval::try_from(std::time::Duration::from_secs(1u64))
            .expect("c9d73cab");
        let task = super::spawn_interval_task(Some(interval), async || panic!("62839854"))
            .expect("7a86a253");
        assert!(matches!(
            task.join().await,
            Err(super::BackgroundTaskShutdownError::Join(_))
        ));
    }
    #[tokio::test(start_paused = true)]
    async fn stuck_background_task_reaches_shutdown_timeout() {
        let interval = super::StdRunInterval::try_from(std::time::Duration::from_secs(1u64))
            .expect("f797718f");
        let task = super::spawn_interval_task(Some(interval), async || {
            std::future::pending::<()>().await;
        })
        .expect("a58f09dc");
        tokio::task::yield_now().await;
        let timeout = super::StdRequestTimeout::try_from(std::time::Duration::from_secs(1u64))
            .expect("ae1262bb");
        let shutdown = tokio::spawn(task.shutdown(timeout));
        tokio::task::yield_now().await;
        tokio::time::advance(std::time::Duration::from_secs(1u64)).await;
        assert!(matches!(
            shutdown.await.expect("9e76a810"),
            Err(super::BackgroundTaskShutdownError::Timeout)
        ));
    }
    #[tokio::test]
    async fn acquire_permit_distinguishes_available_timeout_and_closed() {
        let retry_after = super::RetryAfterSecs::try_from(3u64).expect("c52d0e93");
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(1usize));
        let permit = super::acquire_permit(
            super::StdArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
            super::StdPermitWaitTimeout::from(std::time::Duration::ZERO),
            retry_after,
        )
        .await
        .expect("e1394cd0");
        let timeout = super::acquire_permit(
            super::StdArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
            super::StdPermitWaitTimeout::from(std::time::Duration::ZERO),
            retry_after,
        )
        .await;
        assert!(matches!(
            timeout,
            Err(super::AcquirePermitError::Timeout(value)) if value == retry_after
        ));
        drop(timeout);
        drop(permit);
        semaphore.close();
        let closed = super::acquire_permit(
            super::StdArcTokioSemaphore::from(semaphore),
            super::StdPermitWaitTimeout::from(std::time::Duration::ZERO),
            retry_after,
        )
        .await;
        assert!(matches!(closed, Err(super::AcquirePermitError::Closed(_))));
        drop(closed);
        assert_eq!(
            http::HeaderValue::try_from(retry_after).expect("cb2a239c"),
            http::HeaderValue::from_static("3")
        );
    }
    #[tokio::test]
    async fn request_id_layer_propagates_existing_and_generated_values() {
        let make_router = || {
            axum::Router::from(super::RequestIdLayer.apply(super::AxumRouter::from(
                axum::Router::new().route(
                    str_constants::SLASH,
                    axum::routing::get(async || http::StatusCode::OK),
                ),
            )))
        };
        let existing = http::HeaderValue::from_static(str_constants::EXISTING_REQUEST_ID);
        let existing_response = tower::ServiceExt::oneshot(
            make_router(),
            axum::extract::Request::builder()
                .uri(str_constants::SLASH)
                .header(
                    str_constants::HTTP_HEADER_NAMES_X_REQUEST_ID,
                    existing.clone(),
                )
                .body(axum::body::Body::empty())
                .expect("319b3cb4"),
        )
        .await
        .expect("d5a0693b");
        assert_eq!(
            existing_response
                .headers()
                .get(str_constants::HTTP_HEADER_NAMES_X_REQUEST_ID),
            Some(&existing)
        );
        assert_eq!(
            existing_response
                .headers()
                .get(str_constants::RUNTIME_CORRELATION_ID_HEADER_NAME),
            Some(&existing)
        );
        let generated_response = tower::ServiceExt::oneshot(
            make_router(),
            axum::extract::Request::builder()
                .uri(str_constants::SLASH)
                .body(axum::body::Body::empty())
                .expect("27ce5fbd"),
        )
        .await
        .expect("4cd32371");
        let generated = generated_response
            .headers()
            .get(str_constants::HTTP_HEADER_NAMES_X_REQUEST_ID)
            .expect("12ed6f85");
        assert_eq!(generated.as_bytes().len(), 36usize);
        assert_eq!(
            generated_response
                .headers()
                .get(str_constants::RUNTIME_CORRELATION_ID_HEADER_NAME),
            Some(generated)
        );
    }
    #[tokio::test]
    async fn security_headers_only_trust_forwarded_proto_when_configured() {
        let make_request = || {
            axum::extract::Request::builder()
                .uri(str_constants::API_V1_TEST)
                .header(str_constants::X_FORWARDED_PROTO, str_constants::HTTPS)
                .body(axum::body::Body::empty())
                .expect("94149bdd")
        };
        let make_router = |trust| {
            let policy = super::HttpContentSecurityPolicy::try_from(
                str_constants::TEST_CONTENT_SECURITY_POLICY.to_owned(),
            )
            .expect("abf8cd24");
            axum::Router::from(
                super::SecurityHeadersLayer::from(trust)
                    .with_content_security_policy(policy)
                    .apply(super::AxumRouter::from(axum::Router::new().route(
                        str_constants::API_V1_TEST,
                        axum::routing::get(async || http::StatusCode::OK),
                    ))),
            )
        };
        let ignored_response = tower::ServiceExt::oneshot(
            make_router(super::ForwardedProtoTrust::Ignore),
            make_request(),
        )
        .await
        .expect("8c89e84f");
        assert!(
            ignored_response
                .headers()
                .get("strict-transport-security")
                .is_none()
        );
        let trusted_response = tower::ServiceExt::oneshot(
            make_router(super::ForwardedProtoTrust::Trust),
            make_request(),
        )
        .await
        .expect("db05c4be");
        assert!(
            trusted_response
                .headers()
                .get("strict-transport-security")
                .is_some()
        );
        assert_eq!(
            trusted_response.headers().get(http::header::CACHE_CONTROL),
            Some(&http::HeaderValue::from_static("no-store"))
        );
        assert_eq!(
            trusted_response.headers().get("x-content-type-options"),
            Some(&http::HeaderValue::from_static("nosniff"))
        );
        assert_eq!(
            trusted_response
                .headers()
                .get(str_constants::CONTENT_SECURITY_POLICY_HEADER),
            Some(&http::HeaderValue::from_static(
                str_constants::TEST_CONTENT_SECURITY_POLICY
            ))
        );
    }
    #[tokio::test]
    async fn security_headers_mark_credentials_as_sensitive() {
        let router = axum::Router::from(
            super::SecurityHeadersLayer::from(super::ForwardedProtoTrust::Ignore).apply(
                super::AxumRouter::from(axum::Router::new().route(
                    str_constants::API_V1_TEST,
                    axum::routing::get(async |headers: http::HeaderMap| {
                        assert!(
                            headers
                                .get(http::header::AUTHORIZATION)
                                .is_some_and(http::HeaderValue::is_sensitive)
                        );
                        (
                            [(
                                http::header::SET_COOKIE,
                                str_constants::TEST_SESSION_COOKIE_HEADER_VALUE,
                            )],
                            http::StatusCode::OK,
                        )
                    }),
                )),
            ),
        );
        let response = tower::ServiceExt::oneshot(
            router,
            axum::extract::Request::builder()
                .uri(str_constants::API_V1_TEST)
                .header(
                    http::header::AUTHORIZATION,
                    str_constants::TEST_BEARER_AUTHORIZATION,
                )
                .body(axum::body::Body::empty())
                .expect("703affc9"),
        )
        .await
        .expect("c975d44e");
        assert!(
            response
                .headers()
                .get(http::header::SET_COOKIE)
                .is_some_and(http::HeaderValue::is_sensitive)
        );
    }
    #[test]
    fn zero_limits_are_rejected() {
        let Err(history_error) = super::StdAsyncRunHistoryMaximumLen::try_from(0usize) else {
            panic!("5500cd77");
        };
        assert_eq!(
            history_error,
            super::StdAsyncRunHistoryMaximumLenTryFromUsizeError
        );
        let Err(timeout_error) = super::StdRequestTimeout::try_from(std::time::Duration::ZERO)
        else {
            panic!("bca83cb0");
        };
        assert_eq!(timeout_error, super::StdRequestTimeoutTryFromDurationError);
    }
}
