mod background_job;
mod batched_cleanup;
mod bounded_read;
mod child_process;
mod client_ip;
mod cors;
mod csp;
mod deduplicating_queue;
mod exclusive_run;
mod execution_plan;
mod fallback;
mod generation_gate;
mod geojson;
mod header_text;
mod health;
mod history;
mod http_header_policy;
mod http_policy;
mod http_status_error;
mod identity_bootstrap;
mod lease_registry;
mod lifecycle;
mod limits;
mod metrics_layer;
mod multipart;
mod notification;
mod observability;
mod observed_error;
mod origin;
mod outbound_url;
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
mod source_selection;
mod trace_context;
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
    TrustedProxyRange, TrustedProxyRangeParseError, TrustedProxyRanges, TrustedProxyRangesError,
    TrustedProxyRangesParseError, TrustedProxyRangesTextRef, parse_trusted_proxy_ranges,
    resolve_client_ip, resolve_header_text,
};
pub use cors::{
    HttpCorsAllowOriginHeaderValues, HttpCorsAllowOriginHeaderValuesError,
    HttpCorsAllowOriginTextRef, parse_cors_allow_origin,
};
pub use csp::{
    HttpCspBuilder, HttpCspDirectiveName, HttpCspDirectiveValue, HttpCspMaximumBytesError,
    HttpCspTokenError,
};
pub use deduplicating_queue::{DeduplicatingQueue, QueuePush, StdQueueMaximum};
pub use exclusive_run::{ExclusiveRun, ExclusiveRunAlreadyActive, ExclusiveRunGuard};
pub use execution_plan::{ExecutionMode, ExecutionReport, execute_plan};
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
pub use identity_bootstrap::{
    IdentityBootstrapDecision, IdentityPresence, IdentityRolePresence, IdentitySpec,
    plan_identity_bootstrap,
};
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
pub use observability::{
    ObservabilityGuard, ObservabilityInitError, OpentelemetryOtlpExporterBuildError,
    OpentelemetrySdkObservabilityShutdownError, ServiceName, TracingSubscriberInitError,
    initialize_service_observability,
};
pub use observed_error::{
    ObservedError, ObservedErrorCode, StdObservedErrorBacktrace, StdPanicLocation,
    TracingObservedErrorSpanTrace,
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
    ResourceUtilizationPercentTryFromU8Error, ResourceUtilizationStatus,
    calculate_resource_utilization,
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
    ServiceTracingFormat, StdServiceRuntimeIoError, TokioServiceRuntime, build_service_runtime,
    wait_for_service_shutdown_signal,
};
pub use single_flight::{
    SingleFlight, SingleFlightAcquire, SingleFlightKey, SingleFlightKeyError, SingleFlightOwner,
    SingleFlightWaitOutcome, SingleFlightWaiter, StdSingleFlightMaximum,
};
pub use source_selection::{SourceSelection, SourceSelectionError, select_sources};
pub use text_policy::{
    BoundedTextPolicyError, FixedLengthAsciiHexText, FixedLengthAsciiHexTextError,
    NonEmptyTrimmedText, RequiredNulFreeBoundedText, UrlSafeTokenPartText,
    UrlSafeTokenPartTextError,
};
pub use text_policy::{
    PasswordLength, PasswordLengthRange, PasswordLengthRangeError, PasswordPolicyViolation,
    PasswordTextRef, validate_password_policy,
};
pub use trace_context::{
    HttpHostRef, HttpMethodRef, HttpOpentelemetryHeaderMapMut, HttpOpentelemetryHeaderMapRef,
    HttpTraceParent, HttpTraceParentError, HttpTraceState, HttpTraceStateError,
    OpentelemetryContext, OutboundTraceContext, ReqwestRequest, ReqwestRequestBuilder,
    extract_remote_trace_context, inject_trace_context,
};
pub use wire_token::{VersionedUrlSafeWireTokenText, VersionedUrlSafeWireTokenTextError};
#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumRouter(axum::Router);
#[derive(Clone, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct ReqwestClient(reqwest::Client);

#[derive(Clone, Copy, Debug)]
pub struct StdReqwestConnectTimeout(std::time::Duration);
#[derive(Clone, Copy, Debug)]
pub struct StdReqwestRequestTimeout(std::time::Duration);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("HTTP client timeout must be greater than zero")]
pub struct StdReqwestTimeoutError;
impl TryFrom<std::time::Duration> for StdReqwestConnectTimeout {
    type Error = StdReqwestTimeoutError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdReqwestTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}
impl TryFrom<std::time::Duration> for StdReqwestRequestTimeout {
    type Error = StdReqwestTimeoutError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdReqwestTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ReqwestClientPolicy {
    connect_timeout: StdReqwestConnectTimeout,
    request_timeout: StdReqwestRequestTimeout,
}
impl ReqwestClientPolicy {
    #[must_use]
    pub const fn new(
        connect_timeout: StdReqwestConnectTimeout,
        request_timeout: StdReqwestRequestTimeout,
    ) -> Self {
        Self {
            connect_timeout,
            request_timeout,
        }
    }
}
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct ReqwestClientBuildError(reqwest::Error);
#[derive(Debug, newtype::FromInner)]
struct TracingHttpClientSpan(tracing::Span);

impl ReqwestClient {
    pub async fn execute(
        &self,
        mut request: ReqwestRequest,
    ) -> Result<ReqwestResponse, ReqwestError> {
        let span = Self::prepare_observed_http_request(&mut request);
        tracing::Instrument::instrument(
            async {
                match self.0.execute(request.into_inner()).await {
                    Ok(response) => {
                        let _client_status_record = tracing::Span::current().record(
                            str_constants::OTEL_HTTP_RESPONSE_STATUS_CODE,
                            response.status().as_u16(),
                        );
                        if response.status().is_server_error() {
                            let _client_error_record = tracing::Span::current().record(
                                str_constants::OTEL_STATUS_CODE,
                                str_constants::OTEL_ERROR_STATUS,
                            );
                        }
                        Ok(ReqwestResponse::from(response))
                    }
                    Err(error) => {
                        let _client_error_record = tracing::Span::current().record(
                            str_constants::OTEL_STATUS_CODE,
                            str_constants::OTEL_ERROR_STATUS,
                        );
                        Err(ReqwestError::from(error))
                    }
                }
            },
            span.0,
        )
        .await
    }

    #[allow(clippy::single_call_fn)] // shared preparation keeps production execution and deterministic propagation tests on the same implementation
    fn prepare_observed_http_request(request: &mut ReqwestRequest) -> TracingHttpClientSpan {
        let method = request.method().to_string();
        let host = request
            .host()
            .map_or_else(String::new, |value| value.to_string());
        let span = tracing::info_span!(
            "http.client",
            otel.kind = "client",
            otel.name = tracing::field::Empty,
            otel.status_code = tracing::field::Empty,
            "http.request.method" = %method,
            "server.address" = %host,
            "http.response.status_code" = tracing::field::Empty,
        );
        let _client_name_record =
            span.record(str_constants::OTEL_NAME, format_args!("{method} {host}"));
        inject_trace_context(
            &OpentelemetryContext::from(tracing_opentelemetry::OpenTelemetrySpanExt::context(
                &span,
            )),
            request.headers_mut(),
        );
        TracingHttpClientSpan::from(span)
    }

    pub fn try_new(policy: ReqwestClientPolicy) -> Result<Self, ReqwestClientBuildError> {
        reqwest::Client::builder()
            .connect_timeout(policy.connect_timeout.0)
            .timeout(policy.request_timeout.0)
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map(Self)
            .map_err(ReqwestClientBuildError)
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
#[derive(Debug, newtype::FromInner)]
pub struct TokioTcpListener(tokio::net::TcpListener);

#[derive(Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
pub struct HttpErrorCode(&'static str);
#[derive(Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
pub struct HttpErrorType(&'static str);
#[derive(Clone, Copy, Debug)]
pub struct HttpErrorTelemetry {
    error_code: HttpErrorCode,
    error_type: HttpErrorType,
}
#[derive(Clone, Debug, newtype::Display, newtype::FromInner)]
struct StdHttpErrorBacktrace(Box<str>);
#[derive(Clone, Debug, newtype::Display, newtype::FromInner)]
struct StdHttpErrorChain(Box<str>);
#[derive(Clone, Debug, newtype::Display, newtype::FromInner)]
struct TracingHttpSpanTrace(Box<str>);
#[derive(Clone, Debug)]
pub struct HttpErrorDiagnostic {
    backtrace: StdHttpErrorBacktrace,
    error_chain: StdHttpErrorChain,
    span_trace: TracingHttpSpanTrace,
    telemetry: HttpErrorTelemetry,
}
impl HttpErrorDiagnostic {
    #[must_use]
    pub fn capture(
        telemetry: HttpErrorTelemetry,
        error: &(dyn std::error::Error + 'static),
    ) -> Self {
        let current_span = tracing::Span::current();
        let span_trace = current_span.metadata().map_or_else(
            || str_constants::HTTP_SPAN_UNAVAILABLE.to_owned(),
            |metadata| format!("{current_span:?} [{}]", metadata.name()),
        );
        Self {
            backtrace: StdHttpErrorBacktrace::from(
                std::backtrace::Backtrace::force_capture()
                    .to_string()
                    .into_boxed_str(),
            ),
            error_chain: Self::error_chain(error),
            span_trace: TracingHttpSpanTrace::from(span_trace.into_boxed_str()),
            telemetry,
        }
    }

    fn error_chain(error: &(dyn std::error::Error + 'static)) -> StdHttpErrorChain {
        let mut error_chain = error.to_string();
        let mut optional_source = error.source();
        while let Some(source) = optional_source {
            error_chain.push_str(str_constants::HTTP_ERROR_CHAIN_SEPARATOR);
            error_chain.push_str(source.to_string().as_str());
            optional_source = source.source();
        }
        StdHttpErrorChain::from(error_chain.into_boxed_str())
    }

    #[must_use]
    pub fn from_observed<Source>(error_type: HttpErrorType, error: &ObservedError<Source>) -> Self
    where
        Source: std::error::Error + 'static,
    {
        Self {
            backtrace: StdHttpErrorBacktrace::from(error.backtrace().to_string().into_boxed_str()),
            error_chain: Self::error_chain(error),
            span_trace: TracingHttpSpanTrace::from(error.span_trace().to_string().into_boxed_str()),
            telemetry: HttpErrorTelemetry::new(
                error_type,
                HttpErrorCode::from(error.error_code().get()),
            ),
        }
    }
}
#[derive(Debug, thiserror::Error)]
#[error("{}", str_constants::HTTP_ERROR_WITHOUT_DIAGNOSTIC_CONTEXT)]
struct HttpErrorWithoutDiagnosticContext;
impl HttpErrorTelemetry {
    #[must_use]
    pub const fn new(error_type: HttpErrorType, error_code: HttpErrorCode) -> Self {
        Self {
            error_code,
            error_type,
        }
    }
}
#[derive(Clone, Debug)]
pub struct HttpRequestSpanConfig {
    server_address: StdSocketAddr,
    service_name: ServiceName,
    trusted_proxy_ranges: TrustedProxyRanges,
}
impl HttpRequestSpanConfig {
    #[must_use]
    pub const fn new(
        service_name: ServiceName,
        server_address: StdSocketAddr,
        trusted_proxy_ranges: TrustedProxyRanges,
    ) -> Self {
        Self {
            server_address,
            service_name,
            trusted_proxy_ranges,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct RequestIdLayer {
    span_config: Option<HttpRequestSpanConfig>,
}
impl RequestIdLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter::from(router.0.layer(RequestIdTowerLayer {
            span_config: self.span_config,
        }))
    }

    #[must_use]
    pub const fn with_span_config(span_config: HttpRequestSpanConfig) -> Self {
        Self {
            span_config: Some(span_config),
        }
    }
}
#[derive(Clone, Debug)]
struct RequestIdTowerLayer {
    span_config: Option<HttpRequestSpanConfig>,
}
#[derive(Clone, Debug)]
struct RequestIdService<Service> {
    inner: Service,
    span_config: Option<HttpRequestSpanConfig>,
}
impl<Service> tower::Layer<Service> for RequestIdTowerLayer {
    type Service = RequestIdService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        RequestIdService {
            inner,
            span_config: self.span_config.clone(),
        }
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
        let remote_context =
            extract_remote_trace_context(HttpOpentelemetryHeaderMapRef::from(req.headers()));
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
        let matched_route = req
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .map(axum::extract::MatchedPath::as_str);
        let route = matched_route.unwrap_or(str_constants::HTTP_METRICS_UNMATCHED_PATH);
        let safe_url_path = matched_route.filter(|matched_path| {
            !matched_path.contains('{') && *matched_path == req.uri().path()
        });
        let client_address = self.span_config.as_ref().and_then(|span_config| {
            req.extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|connect_info| {
                    resolve_client_ip(
                        HttpHeaderMapRef::from(req.headers()),
                        StdSocketAddr::from(connect_info.0),
                        &span_config.trusted_proxy_ranges,
                    )
                })
        });
        let span = tracing::info_span!(
            "http.request",
            otel.kind = "server",
            otel.name = tracing::field::Empty,
            otel.status_code = tracing::field::Empty,
            request_id = %request_id_and_header_value.0,
            "http.request.method" = %req.method(),
            "http.route" = %route,
            "http.response.status_code" = tracing::field::Empty,
            "url.path" = tracing::field::Empty,
            "error.type" = tracing::field::Empty,
            error_code = tracing::field::Empty,
            "server.address" = tracing::field::Empty,
            "client.address" = tracing::field::Empty,
            trace_id = tracing::field::Empty,
            span_id = tracing::field::Empty,
            "service.name" = tracing::field::Empty,
        );
        let _server_name_record = span.record(
            str_constants::OTEL_NAME,
            format_args!("{} {route}", req.method()),
        );
        if let Some(path) = safe_url_path {
            let _url_path_record = span.record(str_constants::OTEL_URL_PATH, path);
        }
        if let Some(span_config) = &self.span_config {
            let _server_address_record = span.record(
                str_constants::OTEL_SERVER_ADDRESS,
                tracing::field::display(span_config.server_address),
            );
            let _service_name_record = span.record(
                str_constants::OTEL_SERVICE_NAME,
                tracing::field::display(&span_config.service_name),
            );
        }
        if let Some(address) = client_address {
            let _client_address_record = span.record(
                str_constants::OTEL_CLIENT_ADDRESS,
                tracing::field::display(address),
            );
        }
        if let Err(error) = tracing_opentelemetry::OpenTelemetrySpanExt::set_parent(
            &span,
            (*remote_context).clone(),
        ) {
            tracing::warn!(error = %error, "failed to attach remote OpenTelemetry parent");
        }
        let span_context = tracing_opentelemetry::OpenTelemetrySpanExt::context(&span);
        let opentelemetry_span = opentelemetry::trace::TraceContextExt::span(&span_context);
        let trace_id = opentelemetry_span.span_context().trace_id().to_string();
        let span_id = opentelemetry_span.span_context().span_id().to_string();
        let request_id = request_id_and_header_value.0.clone();
        let http_method = req.method().clone();
        let http_route = route.to_owned();
        let service_name = self
            .span_config
            .as_ref()
            .map_or_else(String::new, |config| config.service_name.to_string());
        let _trace_id_record = span.record(str_constants::OTEL_TRACE_ID, trace_id.as_str());
        let _span_id_record = span.record(str_constants::OTEL_SPAN_ID, span_id.as_str());
        let _previous_extension_request_id =
            req.extensions_mut().insert(request_id_and_header_value.0);
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(tracing::Instrument::instrument(
            async move {
                let mut response = response_future.await?;
                let _server_status_record = tracing::Span::current().record(
                    str_constants::OTEL_HTTP_RESPONSE_STATUS_CODE,
                    response.status().as_u16(),
                );
                if response.status().is_server_error() {
                    let _server_error_record = tracing::Span::current().record(
                        str_constants::OTEL_STATUS_CODE,
                        str_constants::OTEL_ERROR_STATUS,
                    );
                }
                if response.status().is_client_error() || response.status().is_server_error() {
                    let default_error_telemetry = if response.status().is_server_error() {
                        HttpErrorTelemetry::new(
                            HttpErrorType::from(str_constants::OTEL_HTTP_SERVER_ERROR_TYPE),
                            HttpErrorCode::from(str_constants::OTEL_HTTP_5XX_ERROR_CODE),
                        )
                    } else {
                        HttpErrorTelemetry::new(
                            HttpErrorType::from(str_constants::OTEL_HTTP_CLIENT_ERROR_TYPE),
                            HttpErrorCode::from(str_constants::OTEL_HTTP_4XX_ERROR_CODE),
                        )
                    };
                    let optional_diagnostic = response.extensions().get::<HttpErrorDiagnostic>();
                    let error_telemetry = optional_diagnostic
                        .map(|diagnostic| diagnostic.telemetry)
                        .or_else(|| response.extensions().get::<HttpErrorTelemetry>().copied())
                        .unwrap_or(default_error_telemetry);
                    let _error_type_record = tracing::Span::current().record(
                        str_constants::OTEL_ERROR_TYPE,
                        tracing::field::display(error_telemetry.error_type),
                    );
                    let _error_code_record = tracing::Span::current().record(
                        str_constants::OTEL_ERROR_CODE,
                        tracing::field::display(error_telemetry.error_code),
                    );
                    if response.status().is_server_error() {
                        let mut fallback_diagnostic = None;
                        let diagnostic = optional_diagnostic.map_or_else(
                            || {
                                &*fallback_diagnostic.insert(HttpErrorDiagnostic::capture(
                                    error_telemetry,
                                    &HttpErrorWithoutDiagnosticContext,
                                ))
                            },
                            |diagnostic| diagnostic,
                        );
                        tracing::error!(
                            request_id = %request_id,
                            trace_id = %trace_id,
                            service_name = %service_name,
                            http_route = %http_route,
                            http_method = %http_method,
                            http_status = response.status().as_u16(),
                            error_code = %error_telemetry.error_code,
                            error_type = %error_telemetry.error_type,
                            error_chain = %diagnostic.error_chain,
                            backtrace = %diagnostic.backtrace,
                            span_trace = %diagnostic.span_trace,
                            duration_ms = started_at.elapsed().as_millis(),
                            "{}",
                            str_constants::HTTP_REQUEST_FAILED
                        );
                    }
                }
                if !response.status().is_server_error() {
                    tracing::info!(
                        status = response.status().as_u16(),
                        duration_ms = started_at.elapsed().as_millis(),
                        "http request completed"
                    );
                }
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
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct RequestTimeoutLayer(StdRequestTimeout);

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner)]
struct StdRequestTimeoutMessage(&'static str);

#[derive(Debug, serde::Serialize)]
struct RequestTimeoutBody {
    error: StdRequestTimeoutMessage,
}
impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter::from(router.0.layer(RequestTimeoutTowerLayer::from(self.0)))
    }
}
#[derive(Clone, Copy, Debug, newtype::FromInner)]
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
                            error: StdRequestTimeoutMessage::from(str_constants::REQUEST_TIMEOUT),
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
        AxumRouter::from(router.0.layer(SecurityHeadersTowerLayer {
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
                http::HeaderValue::from_static(str_constants::SAME_ORIGIN),
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
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct StdServeIoError(std::io::Error);
#[derive(Debug, thiserror::Error)]
pub enum ServeWithGracefulShutdownError {
    #[error("server failed: {0}")]
    Serve(#[source] StdServeIoError),
    #[error("{}", str_constants::SERVER_GRACEFUL_SHUTDOWN_TIMED_OUT)]
    ShutdownTimeout,
}
#[must_use]
pub fn add_status_route(router: AxumRouter) -> AxumRouter {
    AxumRouter::from(router.0.route(
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
    const HTTP_ERROR_EVENT_REQUIRED_FIELD_MASK: u16 = (1u16 << 11u16) - 1u16;
    #[derive(Clone, Debug)]
    struct HttpErrorEventCapture {
        error_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        field_mask: std::sync::Arc<std::sync::atomic::AtomicU16>,
    }
    impl<Subscriber> tracing_subscriber::Layer<Subscriber> for HttpErrorEventCapture
    where
        Subscriber: tracing::Subscriber,
    {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _context: tracing_subscriber::layer::Context<'_, Subscriber>,
        ) {
            if *event.metadata().level() != tracing::Level::ERROR {
                return;
            }
            let _previous_count = self
                .error_count
                .fetch_add(1usize, std::sync::atomic::Ordering::SeqCst);
            let mut visitor = HttpErrorEventFieldVisitor::default();
            event.record(&mut visitor);
            let _previous_mask = self
                .field_mask
                .fetch_or(visitor.mask, std::sync::atomic::Ordering::SeqCst);
        }
    }
    #[derive(Debug, Default)]
    struct HttpErrorEventFieldVisitor {
        mask: u16,
    }
    impl HttpErrorEventFieldVisitor {
        fn record_field(&mut self, field: &tracing::field::Field) {
            let bit = match field.name() {
                "request_id" => 1u16 << 0u16,
                "trace_id" => 1u16 << 1u16,
                "service_name" => 1u16 << 2u16,
                "http_route" => 1u16 << 3u16,
                "http_method" => 1u16 << 4u16,
                "http_status" => 1u16 << 5u16,
                "error_code" => 1u16 << 6u16,
                "error_type" => 1u16 << 7u16,
                "error_chain" => 1u16 << 8u16,
                "backtrace" => 1u16 << 9u16,
                "span_trace" => 1u16 << 10u16,
                _other => 0u16,
            };
            self.mask |= bit;
        }
    }
    impl tracing::field::Visit for HttpErrorEventFieldVisitor {
        fn record_debug(&mut self, field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {
            self.record_field(field);
        }
        fn record_u64(&mut self, field: &tracing::field::Field, _value: u64) {
            self.record_field(field);
        }
    }
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
            axum::Router::from(
                super::RequestIdLayer::default().apply(super::AxumRouter::from(
                    axum::Router::new().route(
                        str_constants::SLASH,
                        axum::routing::get(async || http::StatusCode::OK),
                    ),
                )),
            )
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
    #[tokio::test(flavor = "current_thread")]
    async fn request_span_uses_remote_parent_and_server_kind() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let tracer =
            opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "server-runtime-test");
        let subscriber = tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            tracing_opentelemetry::layer().with_tracer(tracer),
        );
        let dispatch = tracing::Dispatch::new(subscriber);
        let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
        let trusted_proxy_ranges = super::TrustedProxyRanges::try_from(vec![
            super::TrustedProxyRange::try_from(str_constants::VALUE_127_0_0_1_32.to_owned())
                .expect("0bb46390"),
        ])
        .expect("04cbe253");
        let router = axum::Router::from(
            super::RequestIdLayer::with_span_config(super::HttpRequestSpanConfig::new(
                super::ServiceName::from("server-runtime-test"),
                super::StdSocketAddr::from(
                    "127.0.0.1:8080"
                        .parse::<std::net::SocketAddr>()
                        .expect("773561fe"),
                ),
                trusted_proxy_ranges,
            ))
            .apply(super::AxumRouter::from(axum::Router::new().route(
                "/users/{user_id}",
                axum::routing::get(async || http::StatusCode::OK),
            ))),
        );
        let mut request = axum::extract::Request::builder()
            .uri("/users/42")
            .header(
                str_constants::TRACEPARENT,
                str_constants::TRACEPARENT_TEST_VALUE,
            )
            .header(
                str_constants::RUNTIME_FORWARDED_FOR_HEADER_NAME,
                str_constants::VALUE_203_0_113_1,
            )
            .body(axum::body::Body::empty())
            .expect("f56d84cc");
        let _previous_connect_info = request.extensions_mut().insert(axum::extract::ConnectInfo(
            "127.0.0.1:45000"
                .parse::<std::net::SocketAddr>()
                .expect("0f4a8de7"),
        ));
        let response = tower::ServiceExt::oneshot(router, request)
            .await
            .expect("20b587e3");
        assert_eq!(response.status(), http::StatusCode::OK);
        drop(response);
        tracer_provider.force_flush().expect("8f53d724");
        let spans = exporter.get_finished_spans().expect("88d108d2");
        let request_span = spans
            .iter()
            .find(|span| span.name == "GET /users/{user_id}")
            .expect("fc30b586");
        let expected_trace_id = str_constants::TRACEPARENT_TEST_VALUE
            .get(3usize..35usize)
            .expect("34620ae8");
        let expected_parent_span_id = str_constants::TRACEPARENT_TEST_VALUE
            .get(36usize..52usize)
            .expect("9c70ecdf");
        assert_eq!(
            request_span.span_context.trace_id().to_string(),
            expected_trace_id
        );
        assert_eq!(
            request_span.parent_span_id.to_string(),
            expected_parent_span_id
        );
        assert!(request_span.parent_span_is_remote);
        assert_eq!(
            request_span.span_kind,
            opentelemetry::trace::SpanKind::Server
        );
        let attribute = |key| {
            request_span
                .attributes
                .iter()
                .find(|attribute| attribute.key.as_str() == key)
                .map(|attribute| attribute.value.to_string())
        };
        assert_eq!(attribute("http.request.method").as_deref(), Some("GET"));
        assert_eq!(attribute("http.route").as_deref(), Some("/users/{user_id}"));
        assert_eq!(
            attribute(str_constants::OTEL_HTTP_RESPONSE_STATUS_CODE).as_deref(),
            Some("200")
        );
        assert_eq!(
            attribute(str_constants::OTEL_SERVER_ADDRESS).as_deref(),
            Some("127.0.0.1:8080")
        );
        assert_eq!(
            attribute(str_constants::OTEL_CLIENT_ADDRESS).as_deref(),
            Some(str_constants::VALUE_203_0_113_1)
        );
        assert_eq!(
            attribute(str_constants::OTEL_SERVICE_NAME).as_deref(),
            Some("server-runtime-test")
        );
        assert_eq!(
            attribute(str_constants::OTEL_TRACE_ID).as_deref(),
            Some(expected_trace_id)
        );
        assert_eq!(
            attribute(str_constants::OTEL_SPAN_ID).as_deref(),
            Some(request_span.span_context.span_id().to_string().as_str())
        );
        assert_eq!(attribute(str_constants::OTEL_URL_PATH), None);
        tracer_provider.shutdown().expect("d478940b");
    }
    #[tokio::test(flavor = "current_thread")]
    async fn request_span_limits_url_path_and_records_error_telemetry() {
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let tracer =
            opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "http-span-test");
        let subscriber = tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            tracing_opentelemetry::layer().with_tracer(tracer),
        );
        let dispatch = tracing::Dispatch::new(subscriber);
        let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
        let router = axum::Router::from(super::RequestIdLayer::default().apply(
            super::AxumRouter::from(axum::Router::new().route(
                "/status",
                axum::routing::get(async || {
                    let mut response = axum::response::IntoResponse::into_response(
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                    );
                    let _previous =
                        response
                            .extensions_mut()
                            .insert(super::HttpErrorTelemetry::new(
                                super::HttpErrorType::from("persistence.error"),
                                super::HttpErrorCode::from("database_unavailable"),
                            ));
                    response
                }),
            )),
        ));
        let status_response = tower::ServiceExt::oneshot(
            router.clone(),
            axum::extract::Request::builder()
                .uri("/status")
                .body(axum::body::Body::empty())
                .expect("bd141981"),
        )
        .await
        .expect("22fb2978");
        assert_eq!(
            status_response.status(),
            http::StatusCode::INTERNAL_SERVER_ERROR
        );
        drop(status_response);
        let missing_response = tower::ServiceExt::oneshot(
            router,
            axum::extract::Request::builder()
                .uri("/missing/private-123")
                .body(axum::body::Body::empty())
                .expect("18a1dc0e"),
        )
        .await
        .expect("4dca0c87");
        assert_eq!(missing_response.status(), http::StatusCode::NOT_FOUND);
        drop(missing_response);
        tracer_provider.force_flush().expect("38b83256");
        let spans = exporter.get_finished_spans().expect("72d79c7e");
        let status_span = spans
            .iter()
            .find(|span| span.name == "GET /status")
            .expect("6e0f3748");
        let status_attribute = |key| {
            status_span
                .attributes
                .iter()
                .find(|attribute| attribute.key.as_str() == key)
                .map(|attribute| attribute.value.to_string())
        };
        assert_eq!(
            status_attribute(str_constants::OTEL_URL_PATH).as_deref(),
            Some("/status")
        );
        assert_eq!(
            status_attribute(str_constants::OTEL_ERROR_TYPE).as_deref(),
            Some("persistence.error")
        );
        assert_eq!(
            status_attribute(str_constants::OTEL_ERROR_CODE).as_deref(),
            Some("database_unavailable")
        );
        let unmatched_span = spans
            .iter()
            .find(|span| span.name == "GET __unmatched__")
            .expect("aa6097d2");
        assert!(
            unmatched_span
                .attributes
                .iter()
                .all(|attribute| attribute.value.to_string() != "/missing/private-123")
        );
        assert!(
            unmatched_span
                .attributes
                .iter()
                .all(|attribute| attribute.key.as_str() != str_constants::OTEL_URL_PATH)
        );
        let unmatched_attribute = |key| {
            unmatched_span
                .attributes
                .iter()
                .find(|attribute| attribute.key.as_str() == key)
                .map(|attribute| attribute.value.to_string())
        };
        assert_eq!(
            unmatched_attribute(str_constants::OTEL_ERROR_TYPE).as_deref(),
            Some(str_constants::OTEL_HTTP_CLIENT_ERROR_TYPE)
        );
        assert_eq!(
            unmatched_attribute(str_constants::OTEL_ERROR_CODE).as_deref(),
            Some(str_constants::OTEL_HTTP_4XX_ERROR_CODE)
        );
        tracer_provider.shutdown().expect("a4f89d4d");
    }
    #[tokio::test(flavor = "current_thread")]
    async fn http_boundary_emits_one_complete_error_event_only_for_server_errors() {
        #[derive(Debug, thiserror::Error)]
        #[error("boundary test operation failed")]
        struct BoundaryTestError {
            #[source]
            source: std::io::Error,
        }
        let error_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0usize));
        let field_mask = std::sync::Arc::new(std::sync::atomic::AtomicU16::new(0u16));
        let subscriber = tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            HttpErrorEventCapture {
                error_count: std::sync::Arc::clone(&error_count),
                field_mask: std::sync::Arc::clone(&field_mask),
            },
        );
        let dispatch = tracing::Dispatch::new(subscriber);
        let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
        let diagnostic = super::HttpErrorDiagnostic::capture(
            super::HttpErrorTelemetry::new(
                super::HttpErrorType::from("boundary.test"),
                super::HttpErrorCode::from("boundary_failed"),
            ),
            &BoundaryTestError {
                source: std::io::Error::other("nested source"),
            },
        );
        assert!(
            diagnostic
                .error_chain
                .0
                .contains("boundary test operation failed: nested source")
        );
        assert!(!diagnostic.backtrace.0.to_string().is_empty());
        assert!(!diagnostic.span_trace.0.is_empty());
        let server_error_diagnostic = diagnostic.clone();
        let router = axum::Router::from(
            super::RequestIdLayer::with_span_config(super::HttpRequestSpanConfig::new(
                super::ServiceName::from("boundary-test"),
                super::StdSocketAddr::from(
                    "127.0.0.1:8080"
                        .parse::<std::net::SocketAddr>()
                        .expect("c74109ca"),
                ),
                super::TrustedProxyRanges::default(),
            ))
            .apply(super::AxumRouter::from(
                axum::Router::new()
                    .route(
                        "/failure",
                        axum::routing::get(move || {
                            let response_diagnostic = server_error_diagnostic.clone();
                            async move {
                                let mut response = axum::response::IntoResponse::into_response(
                                    http::StatusCode::INTERNAL_SERVER_ERROR,
                                );
                                let _previous =
                                    response.extensions_mut().insert(response_diagnostic);
                                response
                            }
                        }),
                    )
                    .route(
                        "/invalid",
                        axum::routing::get(async || http::StatusCode::UNPROCESSABLE_ENTITY),
                    ),
            )),
        );
        let server_error_response = tower::ServiceExt::oneshot(
            router.clone(),
            axum::extract::Request::builder()
                .uri("/failure")
                .body(axum::body::Body::empty())
                .expect("2b710c82"),
        )
        .await
        .expect("33c72c1c");
        assert_eq!(
            server_error_response.status(),
            http::StatusCode::INTERNAL_SERVER_ERROR
        );
        drop(server_error_response);
        assert_eq!(
            error_count.load(std::sync::atomic::Ordering::SeqCst),
            1usize
        );
        assert_eq!(
            field_mask.load(std::sync::atomic::Ordering::SeqCst),
            HTTP_ERROR_EVENT_REQUIRED_FIELD_MASK
        );
        let client_error_response = tower::ServiceExt::oneshot(
            router,
            axum::extract::Request::builder()
                .uri("/invalid")
                .body(axum::body::Body::empty())
                .expect("b362c5d1"),
        )
        .await
        .expect("e271f216");
        assert_eq!(
            client_error_response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        drop(client_error_response);
        assert_eq!(
            error_count.load(std::sync::atomic::Ordering::SeqCst),
            1usize
        );
    }
    #[test]
    fn observed_client_preparation_injects_context_and_creates_child_span() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();
        let tracer = opentelemetry::trace::TracerProvider::tracer(
            &tracer_provider,
            "server-runtime-client-test",
        );
        let subscriber = tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            tracing_opentelemetry::layer().with_tracer(tracer),
        );
        let dispatch = tracing::Dispatch::new(subscriber);
        let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
        let url = reqwest::Url::parse(str_constants::HTTPS_EXAMPLE_COM).expect("a0c9b8a8");
        let mut request =
            super::ReqwestRequest::from(reqwest::Request::new(http::Method::GET, url));
        let root_span = tracing::info_span!("caller");
        let prepared_client_span = root_span
            .in_scope(|| super::ReqwestClient::prepare_observed_http_request(&mut request));
        let prepared_request = request.into_inner();
        assert!(
            prepared_request
                .headers()
                .get(str_constants::TRACEPARENT)
                .is_some()
        );
        drop(prepared_client_span);
        drop(root_span);
        let spans = exporter.get_finished_spans().expect("a472015a");
        let caller_span = spans
            .iter()
            .find(|span| span.name == "caller")
            .expect("87c0e547");
        let exported_client_span = spans
            .iter()
            .find(|span| span.name == "GET example.com")
            .expect("5bfcb617");
        assert_eq!(
            exported_client_span.span_context.trace_id(),
            caller_span.span_context.trace_id()
        );
        assert_eq!(
            exported_client_span.parent_span_id,
            caller_span.span_context.span_id()
        );
        assert_eq!(
            exported_client_span.span_kind,
            opentelemetry::trace::SpanKind::Client
        );
        tracer_provider.shutdown().expect("721ff26e");
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
                .get(str_constants::REFERRER_POLICY),
            Some(&http::HeaderValue::from_static(str_constants::SAME_ORIGIN))
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
