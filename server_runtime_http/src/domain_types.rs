mod batched_cleanup;
mod bounded_read;
mod child_process;
mod client_ip;
mod cors;
mod csp;
mod fallback;
mod geojson;
mod header_text;
mod health;
mod http_client;
mod http_error_diagnostic;
mod http_header_policy;
mod http_policy;
mod http_status_error;
mod lifecycle;
mod limits;
mod metrics_layer;
mod multipart;
mod notification;
mod origin;
mod outbound_url;
mod path_policy;
mod pg_rate_limit;
mod redacted_url;
mod request_id;
mod request_timeout;
mod secure_cookie;
mod security_headers;
mod service;
mod service_runtime;
mod trace_context;
mod wire_token;
pub use batched_cleanup::{
    CleanupBatchCount, CleanupBatchSize, CleanupBatchSizeError, CleanupCompletion,
    CleanupContinuation, CleanupReport, CleanupRows, run_batched_cleanup,
};
pub use bounded_read::{
    BoundedBytes, BoundedJsonReadError, BoundedJsonText, BoundedReadConcurrencyArcSemaphore,
    BoundedReadConcurrencyMaximumNonZeroUsize, BoundedReadError, BoundedReadFromUtf8Error,
    BoundedReadIoError, BoundedReadMaximumBytes, BoundedText, IoErrorPresenceDisposition, PathRef,
    ReqwestError, ReqwestResponse, SerdeJsonError, classify_not_found_io_error, parse_bounded_json,
    read_bounded_file, read_bounded_file_async, read_bounded_http_response,
    read_bounded_json_file_async, read_bounded_json_http_response,
};
pub use child_process::{
    ChildDiagnostic, ChildDiagnosticMaximumNonZeroUsize, ChildExitStatus, ChildProcessCompletion,
    ChildProcessError, ChildProcessId, ChildProcessIoError, ChildProcessReport,
    ChildProcessReports, ChildProcessSet, ChildProcessSetError, ChildProcessSetMaximumNonZeroUsize,
    ChildProcessSucceeded, ChildProcessSupervisor, TokioChildProcess, TokioChildProcessJoinError,
};
pub use client_ip::{
    ClientAddrParseError, ClientSocketAddr, HttpHeaderMapRef, ParseIntError, ResolvedClientIpAddr,
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
pub use fallback::{
    FallbackResponseMode, HttpAcceptHeaderMaximumBytes, HttpFallbackApiPrefixRef,
    HttpFallbackMetricsPathRef, HttpFallbackRequestPathRef, HttpOptionalAcceptHeaderRef,
    fallback_response_mode,
};
pub use geojson::{GeoJsonDocumentText, GeoJsonValidationError, SerdeJsonGeoJsonError};
pub use header_text::{
    HttpHeaderName, HttpHeaderTextBytes, HttpHeaderTextMaximumBytes,
    HttpHeaderTextMaximumBytesError, HttpHeaderTextRef, HttpHeaderTextResolution,
};
pub use health::{
    HealthComponentStatus, HealthProbeSucceeded, HealthProbeTimeoutDuration, HealthReadiness,
    HealthSnapshot, ServiceLivenessSnapshot, add_health_routes, run_health_probe,
};
pub use http_client::{
    ReqwestClient, ReqwestClientBuildError, ReqwestClientPolicy, ReqwestConnectTimeoutDuration,
    ReqwestRequestTimeoutDuration, StdReqwestTimeoutError,
};
pub use http_error_diagnostic::{
    HttpErrorCode, HttpErrorDiagnostic, HttpErrorTelemetry, HttpErrorType,
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
pub use lifecycle::{
    BackgroundTask, BackgroundTaskOutcome, BackgroundTaskShutdownError, RequestTimeoutDuration,
    RunIntervalDuration, StdRequestTimeoutTryFromDurationError, StdRunIntervalTryFromDurationError,
    TokioAbortTask, TokioTaskJoinError, abort_and_wait_task, spawn_interval_task,
};
pub use limits::{
    AcquirePermitError, ArcTokioSemaphore, PermitWaitTimeoutDuration, RetryAfterSecs,
    RetryAfterSecsTryFromU64Error, SemaphorePermitCountNonZeroUsize, TokioAcquireError,
    TokioOwnedSemaphorePermit, acquire_permit,
};
pub use metrics_layer::{
    HttpMetricsLayer, HttpMetricsPathCacheMaximum, HttpMetricsPathCacheMaximumTryFromUsizeError,
    MetricsResponseBody, MetricsResponseBodyError,
};
pub use multipart::{
    FileStagingAction, FileStagingDirectoryName, MultipartBytes, MultipartBytesPart,
    MultipartFieldName, MultipartFileName, MultipartPayloadMaximum, MultipartRequestError,
    MultipartTextPart, MultipartTextValue, MultipartUploadRequest, MultipartValueError,
    MultipartValueLength, StoragePathSegment, StoragePathSegmentError, StorageRelativePathBuf,
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
    OutboundIpAddr, OutboundUrlError, OutboundUrlPolicy, OutboundUrlScheme, OutboundUrlTextRef,
    ReqwestOutboundUrl,
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
pub use request_timeout::RequestTimeoutLayer;
pub use secure_cookie::{
    HttpCookieAccess, HttpCookieName, HttpCookieSecure, HttpCookieValue, HttpSecureCookieError,
    HttpSetCookieHeaderValue, StdCookieMaxAgeSeconds, build_secure_strict_cookie,
};
pub use security_headers::{
    ForwardedProtoTrust, HttpContentSecurityPolicy, HttpContentSecurityPolicyError,
    SecurityHeadersLayer,
};
pub use server_observability::service_tracing_format::ServiceTracingFormat;
pub use server_observability::service_tracing_format::capture::*;
pub use server_observability::service_tracing_format::initialization::*;
pub use server_runtime_core::domain_types::*;
pub use service::{
    ServeIoError, ServeWithGracefulShutdownError, ServiceRuntime, TokioTcpListener,
    add_status_route, serve_with_graceful_shutdown,
};
pub use service_runtime::{
    ServiceRuntimeIoError, TokioServiceRuntime, build_service_runtime,
    wait_for_service_shutdown_signal,
};
pub use text_policy::domain_types::{
    BoundedTextPolicyError, FixedLengthAsciiHexText, FixedLengthAsciiHexTextError,
    NonEmptyTrimmedText, RequiredNulFreeBoundedText, UrlSafeTokenPartText,
    UrlSafeTokenPartTextError,
};
pub use text_policy::domain_types::{
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct AxumRouter(axum::Router);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct HttpRequestSpanConfig {
    service_name: ServiceName,
    trusted_proxy_ranges: TrustedProxyRanges,
    server_address: ClientSocketAddr,
}
impl HttpRequestSpanConfig {
    #[must_use]
    pub const fn new(
        service_name: ServiceName,
        server_address: ClientSocketAddr,
        trusted_proxy_ranges: TrustedProxyRanges,
    ) -> Self {
        Self {
            service_name,
            trusted_proxy_ranges,
            server_address,
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
struct RequestIdTowerLayer {
    span_config: Option<HttpRequestSpanConfig>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
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
            constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID,
            constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME,
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
        let route = matched_route.unwrap_or(constants_str::HTTP_METRICS_UNMATCHED_PATH);
        let safe_url_path = matched_route.filter(|matched_path| {
            !matched_path.contains('{') && *matched_path == req.uri().path()
        });
        let client_address = self.span_config.as_ref().and_then(|span_config| {
            req.extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|connect_info| {
                    resolve_client_ip(
                        HttpHeaderMapRef::from(req.headers()),
                        ClientSocketAddr::from(connect_info.0),
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
            constants_str::OTEL_NAME,
            format_args!("{} {route}", req.method()),
        );
        if let Some(path) = safe_url_path {
            let _url_path_record = span.record(constants_str::OTEL_URL_PATH, path);
        }
        if let Some(span_config) = &self.span_config {
            let _server_address_record = span.record(
                constants_str::OTEL_SERVER_ADDRESS,
                tracing::field::display(span_config.server_address),
            );
            let _service_name_record = span.record(
                constants_str::OTEL_SERVICE_NAME,
                tracing::field::display(&span_config.service_name),
            );
        }
        if let Some(address) = client_address {
            let _client_address_record = span.record(
                constants_str::OTEL_CLIENT_ADDRESS,
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
        let _trace_id_record = span.record(constants_str::OTEL_TRACE_ID, trace_id.as_str());
        let _span_id_record = span.record(constants_str::OTEL_SPAN_ID, span_id.as_str());
        let _previous_extension_request_id =
            req.extensions_mut().insert(request_id_and_header_value.0);
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(tracing::Instrument::instrument(
            async move {
                let mut response = response_future.await?;
                let _server_status_record = tracing::Span::current().record(
                    constants_str::OTEL_HTTP_RESPONSE_STATUS_CODE,
                    response.status().as_u16(),
                );
                if response.status().is_server_error() {
                    let _server_error_record = tracing::Span::current().record(
                        constants_str::OTEL_STATUS_CODE,
                        constants_str::OTEL_ERROR_STATUS,
                    );
                }
                if response.status().is_client_error() || response.status().is_server_error() {
                    let default_error_telemetry = if response.status().is_server_error() {
                        HttpErrorTelemetry::new(
                            HttpErrorType::from(constants_str::OTEL_HTTP_SERVER_ERROR_TYPE),
                            HttpErrorCode::from(constants_str::OTEL_HTTP_5XX_ERROR_CODE),
                        )
                    } else {
                        HttpErrorTelemetry::new(
                            HttpErrorType::from(constants_str::OTEL_HTTP_CLIENT_ERROR_TYPE),
                            HttpErrorCode::from(constants_str::OTEL_HTTP_4XX_ERROR_CODE),
                        )
                    };
                    let optional_diagnostic = response.extensions().get::<HttpErrorDiagnostic>();
                    let error_telemetry = optional_diagnostic
                        .map(HttpErrorDiagnostic::telemetry)
                        .or_else(|| response.extensions().get::<HttpErrorTelemetry>().copied())
                        .unwrap_or(default_error_telemetry);
                    let _error_type_record = tracing::Span::current().record(
                        constants_str::OTEL_ERROR_TYPE,
                        tracing::field::display(error_telemetry.error_type()),
                    );
                    let _error_code_record = tracing::Span::current().record(
                        constants_str::OTEL_ERROR_CODE,
                        tracing::field::display(error_telemetry.error_code()),
                    );
                    if response.status().is_server_error() {
                        let mut fallback_diagnostic = None;
                        let diagnostic = optional_diagnostic.map_or_else(
                            || {
                                &*fallback_diagnostic.insert(
                                    http_error_diagnostic::capture_without_context(error_telemetry),
                                )
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
                            error_code = %error_telemetry.error_code(),
                            error_type = %error_telemetry.error_type(),
                            error_chain = %diagnostic.error_chain_text(),
                            error_location = %diagnostic.location(),
                            backtrace = %diagnostic.backtrace(),
                            span_trace = %diagnostic.span_trace(),
                            duration_ms = started_at.elapsed().as_millis(),
                            "{}",
                            constants_str::HTTP_REQUEST_FAILED
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
                    http::HeaderName::from_static(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
                    request_id_and_header_value.1.clone(),
                );
                let _previous_correlation_id = response.headers_mut().insert(
                    http::HeaderName::from_static(
                        constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME,
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
#[cfg(test)]
mod tests;
